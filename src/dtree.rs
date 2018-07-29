use std::collections::{
    HashMap,
    BTreeMap,
    BTreeSet,
};
use types::Literal;
use imper_ast::ValPath;

enum PatternMatchErr {
    Redundant(u16),
    NonExhaustive,
}

#[derive(PartialEq, Eq, Hash)]
pub enum ConstraintValue {
    Finite(u16, u16), // nth out of x options
    Int(isize),
    Str(String),
}

pub struct DTree {
    tree: Tree,
    consted_values: BTreeSet<ValPath>,
}

enum Tree {
    Empty,
    Exit(u16),
    Finite {
        value: ValPath,
        branches: Vec<Tree>,
    },
    Infinite {
        value: ValPath,
        branches: HashMap<ConstraintValue,Tree>,
        default: Box<Tree>
    },
}

impl DTree {
    fn new(num_pats: u16) -> Self {
        DTree {
            tree: Tree::Empty,
            consted_values: BTreeSet::new(),
        }
    }

    fn add_pattern(&mut self, consts: BTreeMap<ValPath, Literal>) {
        let difference: BTreeMap<_,_> = consts.iter().filter(|(k, _)| self.consted_values.contains(k)).collect();
    }
}

impl Tree {
    fn singular(map: &mut BTreeMap<ValPath,ConstraintValue>, mut tail: Tree) -> Self {
        use self::ConstraintValue::*;
        let map2 = BTreeMap::new();
        map2.append(map);
        for (value, consted) in map2.into_iter().rev() {
            match consted {
                Finite(m, n) => {
                    let branches: Vec<_> = (0..n).map(|_| Tree::Empty).collect();
                    branches[m as usize] = tail;
                    tail = Tree::Finite { value, branches };
                },
                Int(_) | Str(_)  => {
                    let branches = HashMap::new();
                    branches.insert(consted, tail);
                    tail = Tree::Infinite { value, branches, default: Box::new(Tree::Empty) };
                },
            }
        }

        tail
    }

    fn modify_with(&mut self, map: &mut BTreeMap<ValPath,ConstraintValue>, exit: u16) {
        use self::Tree::*;
        match *self {
            Empty | Exit(_) => *self = Self::singular(map, Exit(exit)),
            Finite { ref value, ref branches } if map.contains_key(value) => {
                if let ConstraintValue::Finite(n, m) = map.remove(value).unwrap() {
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
                match branches.get(&key) {
                    Some(tree) => tree.modify_with(map, exit),
                    None => { branches.insert(key, Self::singular(map, Exit(exit))); }
                }
            },
            Infinite { ref value, ref mut branches, ref mut default } => {
                for branch in branches.values_mut() {
                    branch.modify_with(map, exit);
                }
                default.modify_with(map, exit);
            }
        }
    }
}