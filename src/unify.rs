//! Hindly-Milner type inference
//! This module implements the unification algorithm for Hindly-Milner type inference

use std::collections::HashMap;
use crate::{
    types::Type,
    error::Error
};

#[cfg(test)]
mod test {
    use self::Type::*;
    use super::*;

    #[test]
    fn test_simple() {
        let mut consts = vec![(Variable(1), Int), (Variable(0), Variable(1))];
        let map = unify(&mut consts).unwrap();
        assert_eq!(map[&0], Variable(1));
        assert_eq!(map[&1], Int);

        let mut t = Function(Box::new(Variable(0)), Box::new(Variable(1)));
        t.substitute_vars(&map);
        assert_eq!(t, Function(Box::new(Int), Box::new(Int)));
    }

    #[test]
    fn test_unify_simple() {
        let mut consts = vec![
            (
                Variable(0),
                Function(Box::new(Variable(1)), Box::new(Variable(6))),
            ),
            (
                Variable(4),
                Function(Box::new(Variable(1)), Box::new(Variable(6))),
            ),
            (
                Variable(2),
                Function(Box::new(Variable(3)), Box::new(Variable(4))),
            ),
            (
                Variable(2),
                Function(
                    Box::new(Int),
                    Box::new(Function(Box::new(Int), Box::new(Int))),
                ),
            ),
            (Variable(3), Int),
        ];
        let map = unify(&mut consts);
        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(map[&3], Int);
        assert_eq!(
            map[&2],
            Function(
                Box::new(Int),
                Box::new(Function(Box::new(Int), Box::new(Int))),
            )
        );
        assert_eq!(map[&4], Function(Box::new(Int), Box::new(Int)));
        assert_eq!(map[&1], Int);
        assert_eq!(map[&6], Int);
        assert_eq!(map[&0], Function(Box::new(Int), Box::new(Int)));
    }

    #[test]
    fn test_unify_fold() {
        // see consts.txt for derivation
        let mut consts = vec![
            (
                Variable(0),
                Function(
                    Box::new(Variable(1)),
                    Box::new(Function(
                        Box::new(Variable(2)),
                        Box::new(Function(Box::new(Variable(3)), Box::new(Variable(4)))),
                    )),
                ),
            ),
            (Variable(3), Sum(0, vec![Variable(5)])),
            (Variable(4), Variable(2)),
            (Variable(3), Sum(0, vec![Variable(6)])),
            (
                Variable(7),
                Function(Box::new(Variable(8)), Box::new(Variable(4))),
            ),
            (Variable(7), Variable(1)),
            (Variable(8), Tuple(vec![Variable(9), Variable(10)])),
            (Variable(9), Variable(6)),
            (
                Variable(11),
                Function(
                    Box::new(Sum(0, vec![Variable(6)])),
                    Box::new(Variable(10)),
                ),
            ),
            (
                Variable(12),
                Function(Box::new(Variable(2)), Box::new(Variable(11))),
            ),
            (
                Variable(13),
                Function(Box::new(Variable(1)), Box::new(Variable(12))),
            ),
            (Variable(13), Variable(0)),
        ];
        let map = unify(&mut consts).unwrap();

        let mut f_type = Variable(1);
        f_type.substitute_vars(&map);
        f_type.generalize_type();
        assert_eq!(
            f_type,
            Function(
                Box::new(Tuple(vec![Generic(0), Generic(1)])),
                Box::new(Generic(1))
            )
        );

        let mut fold_type = Variable(0);
        fold_type.substitute_vars(&map);
        fold_type.generalize_type();
        assert_eq!(
            fold_type,
            Function(
                Box::new(Function(
                    Box::new(Tuple(vec![Generic(0), Generic(1)])),
                    Box::new(Generic(1))
                )),
                Box::new(Function(
                    Box::new(Generic(1)),
                    Box::new(Function(
                        Box::new(Sum(0, vec![Generic(0)])),
                        Box::new(Generic(1))
                    ))
                ))
            )
        );
    }
}

impl Type {
    /// given a map from variables to types, substitute variable types in self with
    /// the corresponding types from map. The substituted-with types can themselves
    /// have substitutable variables so we recures
    /// # REQUIRES
    /// no cycles in substitutions map
    pub fn substitute_vars(&mut self, map: &HashMap<u16, Type>) {
        match *self {
            Type::Int | Type::Bool | Type::String | Type::Unit | Type::Generic(_) | Type::Constructor {..} => (),
            Type::Variable(n) => {
                if let Some(t) = map.get(&n) {
                    *self = t.clone();
                    self.substitute_vars(map);
                }
            }
            Type::Function(ref mut from, ref mut to) => {
                from.substitute_vars(map);
                to.substitute_vars(map);
            }
            Type::Tuple(ref mut v) | Type::Sum(_, ref mut v) => {
                for t in v {
                    t.substitute_vars(map);
                }
            }
        }
    }
}

/// Hindly-Milner unification
pub fn unify(consts: &mut Vec<(Type, Type)>) -> Result<HashMap<u16, Type>, Error> {
    let mut map = HashMap::new();
    // FIXME: don't return immediately at error, keep unifying
    while consts.len() > 0 {
        let (mut tl, mut tr) = consts.pop().unwrap();
        tl.substitute_vars(&map); 
        tr.substitute_vars(&map);
        if tl == tr { continue; }
        match  (tl, tr) {
            (Type::Int, Type::Int)
            | (Type::Bool, Type::Bool)
            | (Type::Unit, Type::Unit)
            | (Type::String, Type::String) => (),
            (Type::Variable(n), Type::Variable(m)) if n == m => (),
            (Type::Variable(n), l) | (l, Type::Variable(n)) => { 
                map.insert(n, l); 
            }
            (Type::Function(from1, to1), Type::Function(from2, to2)) => {
                consts.push((*from1, *from2));
                consts.push((*to1, *to2));
            }
            (Type::Tuple(v), Type::Tuple(u)) => {
                for (x, y) in v.into_iter().zip(u.into_iter()) {
                    consts.push((x, y));
                }
            }
            (Type::Sum(n, v), Type::Sum(m, u)) => {
                if n == m {
                    for (x, y) in v.into_iter().zip(u.into_iter()) {
                        consts.push((x, y));
                    }                
                } else {
                    return Err(Error::TypeMismatch(Type::Sum(n, v), Type::Sum(m, u)));
                }
            }
            // generics are always instantiated to variables before unification
            (Type::Generic(_), _) | (_, Type::Generic(_)) => {
                panic!("Generic not expected in unification")
            }
            (Type::Constructor { .. }, _) | (_, Type::Constructor { .. }) => {
                return Err(Error::ConstructorUnification)
            }
            (t1, t2) => return Err(Error::TypeMismatch(t1, t2)),
        }
    }
    Ok(map)
}