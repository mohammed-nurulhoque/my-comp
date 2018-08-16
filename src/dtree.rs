//! Author Mohammed Nurul Hoque, July 28, 2018
//! This module is part of the translation of the AST to phase 2, a more
//! imperative representation of a program
//! It generates a decision tree from a set of patterns and checks for
//! redundancy and exhaustiveness in the patterns.

use std::{
    mem,
    collections::{
        HashMap,
        BTreeMap,
        BTreeSet,
    }
};
use imper_ast::ValPath;
use imper_ast::ConstraintValue;

pub enum PatternMatchErr { // FIXME: make private and convert to general error
    Redundant(u16),
    NonExhaustive,
}

/// Decision Tree
pub struct WrappedTree {
    pub dtree: DTree,
    /// Set of paths in the matched type that are constrained by the matchings
    consted_values: BTreeSet<ValPath>,
}

pub enum DTree {
    /// existance in final tree implies inexhaustiveness
    Empty,
    /// nth pattern satisfied
    Exit(u16),
    Finite {
        value: ValPath,
        /// branch for each possibility in the finite set
        branches: Vec<DTree>,
    },
    Infinite {
        value: ValPath,
        /// branch for each constrained value
        branches: HashMap<ConstraintValue,DTree>,
        /// default path if no value in map matched
        default: Box<DTree>
    },
}

impl WrappedTree {
    /// a new empty wrapped tree
    pub fn new() -> Self {
        WrappedTree {
            dtree: DTree::Empty,
            consted_values: BTreeSet::new(),
        }
    }

    /// add another pattern to the decision tree, 
    /// the patterns should be added in reverse order
    pub fn add_pattern(&mut self, consts: BTreeMap<ValPath,ConstraintValue>, exit: u16) {
        let mut difference = BTreeMap::new();
        let mut intersect = BTreeMap::new();
        for (k, v) in consts {
            if self.consted_values.contains(&k) {
                intersect.insert(k, v);
            } else {
                difference.insert(k, v);
            }
        }
        self.dtree.modify_with(&mut intersect, exit);
        let mut dtree = DTree::Empty;
        mem::swap(&mut dtree, &mut self.dtree);
        self.dtree = DTree::singular(&difference, dtree);
    }
}

impl DTree {
    /// create a tree the matches the constraints in map and all other branches
    /// are empty
    fn singular(map: &BTreeMap<ValPath,ConstraintValue>, mut tail: DTree) -> Self {
        use self::ConstraintValue::*;
        for (value, consted) in map.iter().rev() {
            match *consted {
                Finite(m, n) => {
                    let mut branches: Vec<_> = (0..n).map(|_| DTree::Empty).collect();
                    branches[m as usize] = tail;
                    tail = DTree::Finite { value: value.clone(), branches };
                },
                Int(_) | Str(_)  => {
                    let mut branches = HashMap::new();
                    branches.insert(consted.clone(), tail);
                    tail = DTree::Infinite { value: value.clone(), branches, default: Box::new(DTree::Empty) };
                },
            }
        }

        tail
    }

    /// modify the tree to match the exit pattern when the constraints in map are met
    /// REQUIRES self matches all patterns after the one in map
    fn modify_with(&mut self, map: &mut BTreeMap<ValPath,ConstraintValue>, exit: u16) {
        use self::DTree::*;
        match *self {
            Empty | Exit(_) => *self = Self::singular(map, Exit(exit)),
            Finite { ref value, ref mut branches } if map.contains_key(value) => {
                if let ConstraintValue::Finite(n, _) = map.remove(value).unwrap() { // !!!
                    branches[n as usize].modify_with(map, exit)
                } else {
                    panic!("infinite & finite contradiction")
                }
            },
            Finite { ref mut branches, .. } => for branch in branches {
                branch.modify_with(map, exit)
            },
            Infinite { ref value, ref mut branches, .. } if map.contains_key(value) => {
                let key = map.remove(value).unwrap();
                if let Some(dtree) = branches.get_mut(&key) {
                    return dtree.modify_with(map, exit)
                }
                branches.insert(key, Self::singular(map, Exit(exit)));
            },
            Infinite { ref mut branches, ref mut default, .. } => {
                for branch in branches.values_mut() {
                    branch.modify_with(map, exit);
                }
                default.modify_with(map, exit);
            }
        }
    }

    pub fn check_ok(&self, counter: &mut Vec<u16>) -> Result<(),PatternMatchErr> {
        use self::DTree::*;
        match *self {
            Empty => Err(PatternMatchErr::NonExhaustive),
            Exit(n) => {
                counter[n as usize] += 1;
                Ok(())
            },
            Finite { ref branches, .. } => branches.iter().map(|b| b.check_ok(counter)).collect(),
            Infinite { ref branches, ref default, .. } => {
                let res: Result<(), PatternMatchErr> = branches.iter().map(|(_, b)| b.check_ok(counter)).collect();
                res?;
                default.check_ok(counter)
            },
        }
    }
}