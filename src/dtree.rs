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

use imper_ast::ConstraintValue;
use imper_ast::ValPath;
use std::collections::{BTreeMap, HashMap};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let tree = DTree::new();
        for set in [
            (
                0,
                [
                    (ValPath::local(vec![0]), ConstraintValue::Finite(0, 2)),
                    (ValPath::local(vec![1]), ConstraintValue::Finite(1, 2)),
                ],
            ),
            (
                1,
                [
                    (ValPath::local(vec![0]), ConstraintValue::Finite(1, 2)),
                    (ValPath::local(vec![1]), ConstraintValue::Finite(0, 2)),
                ],
            ),
            (
                2,
                [(ValPath::local(vec![0]), ConstraintValue::Finite(0, 2))],
            ),
            (3, []),
        ] {
            let map = BTreeMap::new();
            for (i, array) in set {
                map.insert(valpath, constraint);
            }
            tree.add_pattern(maps)
        }
        tree.is_sound_complete(3).unwrap();
    }
}

pub enum PatternMatchErr {
    // FIXME: make private and convert to general error
    Redundant(u16),
    NonExhaustive,
}

#[derive(Clone)]
pub enum DTree {
    /// existance in final tree implies inexhaustiveness
    Empty,
    /// nth pattern satisfied
    Exit(u16),
    Finite {
        /// value to test
        value: ValPath,
        /// branch for each possibility in the finite set
        branches: Vec<DTree>,
    },
    Infinite {
        /// value to test
        value: ValPath,
        /// branch for each constrained value
        branches: HashMap<ConstraintValue, DTree>,
        /// default path if no value in map matched
        default: Box<DTree>,
    },
}

impl DTree {
    pub fn new() -> Self {
        DTree::Empty
    }

    /// create a tree the matches the constraints exiting with tail, else exits with default
    fn singular(
        map: &BTreeMap<ValPath, ConstraintValue>,
        mut tail: DTree,
        default: &DTree,
    ) -> Self {
        use self::ConstraintValue::*;
        for (value, consted) in map.iter().rev() {
            match *consted {
                Finite(m, n) => {
                    let mut branches: Vec<_> = (0..n)
                        .map(|i| {
                            if i == m {
                                DTree::Empty
                            } else {
                                default.clone()
                            }
                        })
                        .collect();
                    branches[m as usize] = tail;
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
    /// REQUIRES exit has higher precedence that patterns in self
    pub fn add_pattern(&mut self, map: &mut BTreeMap<ValPath, ConstraintValue>, exit: u16) {
        use self::DTree::*;
        match *self {
            Empty | Exit(_) => *self = Self::singular(map, Exit(exit), self),
            Finite {
                ref value,
                ref mut branches,
            }
                if map.contains_key(value) =>
            {
                if let ConstraintValue::Finite(n, _) = map.remove(value).unwrap() {
                    // !!!
                    branches[n as usize].add_pattern(map, exit)
                } else {
                    panic!("infinite & finite contradiction")
                }
            }
            Finite {
                ref mut branches, ..
            } => {
                for branch in branches {
                    branch.add_pattern(&mut map.clone(), exit)
                }
            }
            Infinite {
                ref value,
                ref mut branches,
                ..
            }
                if map.contains_key(value) =>
            {
                let key = map.remove(value).unwrap();
                if let Some(dtree) = branches.get_mut(&key) {
                    return dtree.add_pattern(map, exit);
                }
                branches.insert(key, Self::singular(map, Exit(exit), &DTree::Empty));
            }
            Infinite {
                ref mut branches,
                ref mut default,
                ..
            } => {
                for branch in branches.values_mut() {
                    branch.add_pattern(&mut map.clone(), exit);
                }
                default.add_pattern(map, exit);
            }
        }
    }

    fn check_tree(&self, counter: &mut Vec<bool>) -> bool {
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
            Infinite {
                ref branches,
                ref default,
                ..
            } => {
                let res = branches
                    .iter()
                    .map(|(_, b)| b.check_tree(counter))
                    .any(|res| res == true);
                res && default.check_tree(counter)
            }
        }
    }

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
}
