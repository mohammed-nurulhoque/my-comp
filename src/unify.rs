use std::collections::HashMap;
use type_check::Error;
use types::Type;

macro_rules! b {
    ($x: expr) => {
        Box::new($x)
    };
}

#[cfg(test)]
mod test {
    use self::Type::*;
    use super::*;
    #[test]
    fn test_unify() {
        let consts = vec![
            (
                Variable(0),
                Function(b!(Variable(1)), b!(Variable(6))),
            ),
            (
                Variable(4),
                Function(b!(Variable(1)), b!(Variable(6))),
            ),
            (
                Variable(2),
                Function(b!(Variable(3)), b!(Variable(4))),
            ),
            (
                Variable(2),
                Function(
                    b!(Int),
                    b!(Function(b!(Int), b!(Int))),
                ),
            ),
            (Variable(3), Int),
        ];
        let map = unify(consts);
        assert!(map.is_ok());
        let map = map.unwrap();
        assert_eq!(map[&3], Int);
        assert_eq!(
            map[&2],
            Function(
                b!(Int),
                b!(Function(b!(Int), b!(Int))),
            )
        );
        assert_eq!(map[&4], Function(b!(Int), b!(Int)));
        assert_eq!(map[&1], Int);
        assert_eq!(map[&6], Int);
        assert_eq!(map[&0], Function(b!(Int), b!(Int)));
    }

    #[test]
    fn test_substitute_simple() {
        let consts = vec![(Variable(1), Int), (Variable(0), Variable(1))];
        let map = unify(consts).unwrap();
        assert_eq!(map[&0], Variable(1));
        assert_eq!(map[&1], Int);

        let mut t = Function(b!(Variable(0)), b!(Variable(1)));
        t.substitute_type(&map);
        assert_eq!(t, Function(b!(Int), b!(Int)));
    }

    #[test]
    fn test_substitute() {
        let consts = vec![
            (Variable(0), Function(b!(Variable(1)), b!(Function(b!(Variable(2)), b!(Function(b!(Variable(3)), b!(Variable(4)))))))), 
            (Variable(3), Sum(0, b!(Variable(5)))),
            (Variable(4), Variable(2)),
            (Variable(3), Sum(0, b!(Variable(6)))),
            (Variable(7), Function(b!(Variable(8), b!(Variable(4))))),
            (Variable(7), Variable(1)),
            (Variable(8), Tuple(vec![Variable(9), Variable(10)])),
            (Variable(9), Variable(6)),
            (Variable(11), Function(b!()))
            
        ];
        let map = unify(consts).unwrap();
        let mut t = Function(b!(Variable(0)), b!(Variable(1)));
        t.substitute_type(&map);
        assert_eq!(t, Function(b!(Int), b!(Int)));
    }
}

impl Type {
    fn replace_type(&mut self, n: u16, target: &Type) {
        match *self {
            Type::Int | Type::Bool | Type::String | Type::Unit => (),
            Type::Variable(m) => if n == m {
                *self = target.clone()
            },
            Type::Constructor { arg: ref mut t, .. } | Type::Sum(_, ref mut t) => {
                t.replace_type(n, target)
            }
            Type::Function(ref mut from, ref mut to) => {
                from.replace_type(n, target);
                to.replace_type(n, target);
            }
            Type::Tuple(ref mut v) => for t in v {
                t.replace_type(n, target);
            },
            Type::Generic(_) => panic!("generic not expected in replace_type"),
        }
    }

    pub fn substitute_type(&mut self, map: &HashMap<u16, Type>) {
        match *self {
            Type::Int | Type::Bool | Type::String | Type::Unit => (),
            Type::Variable(n) => if map.get(&n).is_some() {
                if let Some(t) = map.get(&n) {
                    *self = t.clone();
                    self.substitute_type(map)
                }
            },
            Type::Constructor { arg: ref mut t, .. } | Type::Sum(_, ref mut t) => {
                t.substitute_type(map)
            }
            Type::Function(ref mut from, ref mut to) => {
                from.substitute_type(map);
                to.substitute_type(map);
            }
            Type::Tuple(ref mut v) => {
                for t in v {
                    t.substitute_type(map);
                }
            }
            Type::Generic(_) => panic!("generic not expected for substitution"),
        }
    }

    fn generalize(&mut self, map: &mut HashMap<u16, u16>) {
        match *self {
            Type::Int | Type::Bool | Type::String | Type::Unit => (),
            Type::Variable(n) => if map.get(&n).is_some() {
                match map.get(&n) {
                    Some(&m) => *self = Type::Generic(m),
                    None => {
                        let len = map.len() as u16;
                        map.insert(n, len);
                        *self = Type::Generic(len)
                    }
                }
            },
            Type::Constructor { arg: ref mut t, .. } | Type::Sum(_, ref mut t) => t.generalize(map),
            Type::Function(ref mut from, ref mut to) => {
                from.generalize(map);
                to.generalize(map);
            }
            Type::Tuple(ref mut v) => {
                for t in v {
                    t.generalize(map);
                }
            }
            Type::Generic(_) => panic!("generic not expected in generalize"),
        }
    }

    pub fn generalize_type(&mut self) {
        self.generalize(&mut HashMap::new())
    }
}

pub fn unify(mut consts: Vec<(Type, Type)>) -> Result<HashMap<u16, Type>, Error> {
    let mut map = HashMap::new();
    while consts.len() > 0 {
        match consts.pop().unwrap() {
            (Type::Int, Type::Int)
            | (Type::Bool, Type::Bool)
            | (Type::Unit, Type::Unit)
            | (Type::String, Type::String) => (),
            (Type::Variable(n), Type::Variable(m)) if n == m => (),
            (Type::Variable(n), l) | (l, Type::Variable(n)) => {
                for (tl, tr) in &mut consts {
                    tl.replace_type(n, &l);
                    tr.replace_type(n, &l);
                }
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
            (Type::Sum(n, s), Type::Sum(m, t)) => if n == m {
                consts.push((*s, *t))
            } else {
                return Err(Error::TypeMismatch(Type::Sum(n, s), Type::Sum(m, t)));
            },
            (Type::Generic(_), _)
            | (_, Type::Generic(_))
            | (Type::Constructor { .. }, _)
            | (_, Type::Constructor { .. }) => return Err(Error::ConstructorUnification),
            (t1, t2) => return Err(Error::TypeMismatch(t1, t2)), // error type mismatch
        }
    }
    Ok(map)
}
