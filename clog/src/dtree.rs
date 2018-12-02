//! Author Mohammed Nurul Hoque, July 28, 2018
//! This module is part of the translation of the AST to phase 2, a more
//! imperative representation of a program
//! It generates a decision tree from a set of patterns and checks for
//! redundancy and exhaustiveness in the patterns.
//!
//! #Algorithm
//!
//! INPUT: a sequence of sets of constraints, where each set defines the
//! value path constraints in a pattern, and the sequnce contains the patterns in order.
//!
//! OUTPUT: a wrappedTree containing the decision tree for the set of patterns.
//!
//! Procedure description:
//! The decision tree for a single constraints set is trivial with
//! the condition that the constraint for the tag of a union tag should precede
//! the constraint for the value of the union which is achieved by making the
//! set lexicographically. A tag has a .0 path while a value has >= .1.
//!
//! For a sequence of constraint, we add them in reverse order with the higher
//! patterns (later inserted) possibly overriding the decision of a lower
//! pattern.
//!
//! When a new pattern p with constraints C is inserted to a tree T
//! For nodes in T starting from root:
//! if node.value v in C:
//!     modify node.branches[v] with C = C\v
//! else:
//!     modify all branches with C
//! When T = exit or empty reached:
//!     replace with signular(C, tail = exit(i), default = T)

use std::collections::{BTreeMap, HashMap};
use crate::{
    imper_ast::ConstraintValue,
    imper_ast::ValPath,
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let all_tests = vec![
            vec![
                /*
                true false -> 0
                false true -> 1
                true  _    -> 2
                _     _    -> 3
                */
                (
                    0,
                    vec![
                        (ValPath::Local(vec![0]), ConstraintValue::Finite(1, 2)),
                        (ValPath::Local(vec![1]), ConstraintValue::Finite(2, 2)),
                    ],
                ),
                (
                    1,
                    vec![
                        (ValPath::Local(vec![0]), ConstraintValue::Finite(2, 2)),
                        (ValPath::Local(vec![1]), ConstraintValue::Finite(1, 2)),
                    ],
                ),
                (
                    2,
                    vec![(ValPath::Local(vec![0]), ConstraintValue::Finite(1, 2))],
                ),
                (3, vec![]),
            ],
            vec![
                /*
                (TWO  13) _ -> 0
                _         5 -> 1
                (ONE   _) _ -> 2
                (THREE _) _ -> 3
                _         _ -> 4
                 */
                (
                    0,
                    vec![
                        (ValPath::Local(vec![0, 0]), ConstraintValue::Finite(2, 3)),
                        (ValPath::Local(vec![0, 1]), ConstraintValue::Int(13)),
                    ],
                ),
                (1, vec![(ValPath::Local(vec![1]), ConstraintValue::Int(5))]),
                (
                    2,
                    vec![(ValPath::Local(vec![0, 0]), ConstraintValue::Finite(1, 3))],
                ),
                (
                    3,
                    vec![(ValPath::Local(vec![0, 0]), ConstraintValue::Finite(3, 3))],
                ),
                (4, vec![]),
            ],
        ];

        for set in all_tests {
            let mut tree = DTree::new();
            let len = set.len() as u16;
            for (i, array) in set.into_iter().rev() {
                let mut map = BTreeMap::new();
                for (valpath, constraint) in array {
                    map.insert(valpath, constraint);
                }
                tree.add_pattern(map, i)
            }
            assert!(tree.is_sound_complete(len).is_ok());
        }
    }
}

pub enum PatternMatchErr {
    // FIXME: make private and convert to general error
    Redundant(u16),
    NonExhaustive,
}

/// A decision tree for pattern matching
#[derive(Clone, Debug)]
pub enum DTree<'input> {
    /// existance in final tree implies inexhaustiveness
    Empty,
    /// nth pattern satisfied
    Exit(u16),
    /// bool or tagged union
    Finite {
        /// value to test
        value: ValPath,
        /// branch for each possibility in the finite set
        branches: Vec<DTree<'input>>,
    },
    /// integer or string
    Infinite {
        /// value to test
        value: ValPath,
        /// branch for each constrained value
        branches: HashMap<ConstraintValue<'input>, DTree<'input>>,
        /// default path if no value in map matched
        default: Box<DTree<'input>>,
    },
}

impl<'input> DTree<'input> {
    pub fn new() -> Self {
        DTree::Empty
    }

    /// create a tree the matches the constraints in map exiting with tail, else exits with default
    fn make_tree(
        map: &BTreeMap<ValPath, ConstraintValue<'input>>,
        mut tail: DTree<'input>,
        default: &DTree<'input>,
    ) -> Self {
        use self::ConstraintValue::*;
        for (value, consted) in map.iter().rev() {
            match *consted {
                Finite(m, n) => {
                    let mut branches: Vec<_> = (1..=n)
                        .map(|i| {
                            if i == m {
                                DTree::Empty
                            } else {
                                default.clone()
                            }
                        })
                        .collect();
                    branches[m as usize - 1] = tail;
                    tail = DTree::Finite {
                        value: value.clone(),
                        branches,
                    };
                }
                Int(_) | Str(_) => {
                    let mut branches = HashMap::new();
                    branches.insert(consted.clone(), tail);
                    tail = DTree::Infinite {
                        value: value.clone(),
                        branches,
                        default: Box::new(default.clone()),
                    };
                }
            }
        }
        tail
    }

    /// modify the tree to match the exit pattern when the constraints in map are met
    /// ### REQUIRES
    /// exit has higher precedence that patterns in self
    pub fn add_pattern(&mut self, mut map: BTreeMap<ValPath, ConstraintValue<'input>>, exit: u16) {
        use self::DTree::*;
        match *self {
            Empty | Exit(_) => *self = Self::make_tree(&map, Exit(exit), self),
            Finite { ref value, ref mut branches } if map.contains_key(value) => {
                if let ConstraintValue::Finite(n, _) = map.remove(value).unwrap() {
                    // !!!
                    branches[n as usize - 1].add_pattern(map, exit)
                } else {
                    panic!("infinite & finite contradiction")
                }
            }
            Finite { ref mut branches, .. } => {
                for branch in branches {
                    branch.add_pattern(map.clone(), exit)
                }
            }
            Infinite { ref value, ref mut branches, .. } if map.contains_key(value) => {
                let key = map.remove(value).unwrap();
                // logically this is
                // if let _ = branches.get_mut() { add pattern } else { insert branch }
                // but branches remains borrowed in else part, hence this structure
                if let Some(dtree) = branches.get_mut(&key) {
                    dtree.add_pattern(map, exit);
                } else {
                    branches.insert(key, Self::make_tree(&map, Exit(exit), &DTree::Empty));
                }
            }
            Infinite { ref mut branches, ref mut default, .. } => {
                for branch in branches.values_mut() {
                    branch.add_pattern(map.clone(), exit);
                }
                default.add_pattern(map, exit);
            }
        }
    }

    /// check whether a decision tree is exhaustive and non-repetitive
    pub fn is_sound_complete(&self, num_pats: u16) -> Result<(), PatternMatchErr> {
        let mut flags = vec![false; num_pats as usize];
        if self.check_tree(&mut flags) {
            for (i, p) in flags.iter().enumerate() {
                if !p {
                    return Err(PatternMatchErr::Redundant(i as u16));
                }
            }
            Ok(())
        } else {
            Err(PatternMatchErr::NonExhaustive)
        }
    }

    /// check that dtree has no empty subtree, and sets flags for exits found
    fn check_tree(&self, counter: &mut [bool]) -> bool {
        use self::DTree::*;
        match *self {
            Empty => false,
            Exit(n) => {
                counter[n as usize] = true;
                true
            }
            Finite { ref branches, .. } => {
                branches.iter().map(|b| b.check_tree(counter)).all(|p| p)
            }
            Infinite { ref branches, ref default, .. } => {
                let res = branches
                    .iter()
                    .map(|(_, b)| b.check_tree(counter))
                    .all(|res| res);
                res && default.check_tree(counter)
            }
        }
    }
}