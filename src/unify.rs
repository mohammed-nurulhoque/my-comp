use std::{
    collections::HashMap,
};
use types::Type;

impl Type {
    fn replace_type(&mut self, n: u16, target: &Type) {
        match *self {
            Type::Variable(m) if n == m => *self = target.clone(),
            Type::Constructor { arg: ref mut t, .. } => t.replace_type(n, target),
            Type::Function(ref mut from, ref mut to) => {
                from.replace_type(n, target);
                to.replace_type(n, target);
            },
            Type::Tuple(ref mut v) | Type::Sum(_, ref mut v) => {
                v.into_iter().map(|t| t.replace_type(n, target));
            },
        }
    }

    fn substitute_type(&mut self, map: HashMap<u16, Type>) {
        match *self {
            Type::Variable(n) => if let Some(t) = map.get(&n) {
                *self = t.clone();
                self.substitute_type(map)
            },
            Type::Constructor { arg: t, .. } => t.substitute_type(map),
            Type::Function(ref mut from, ref mut to) => {
                from.substitute_type(map);
                to.substitute_type(map);
            },
            Type::Tuple(ref mut v) | Type::Sum(_, ref mut v) => {
                v.into_iter().map(|t| t.substitute_type(map));
            },
        }
    }
}

fn unify(consts: Vec<(Type, Type)>) -> HashMap<u16, Type> {
    let mut map = HashMap::new();
    while consts.len() > 0 {
        match consts.pop().unwrap() {
            (Type::Variable(n), Type::Variable(m)) if n == m => (),
            (Type::Variable(n), l) => {
                consts.into_iter().map(|(tl, tr)| {
                    tl.replace_type(n, &l);
                    tr.replace_type(n, &l);
                });
                map.insert(n, l);
            },
            (Type::Function(from1, to1), Type::Function(from2, to2)) => {
                consts.push((*from1, *from2));
                consts.push((*to1, *to2));
            },
            (Type::Tuple(v), Type::Tuple(u)) => {
                v.into_iter().zip(u.into_iter()).map(|(x, y)| consts.push((x, y)));
            },
            (Type::Sum(n, s), Type::Sum(m, t)) if n == m => consts.push((s, t)),
            (Type::Generic(_), _) | (_, Type::Generic(_)) 
            | (Type::Constructor {..}, _) | (_, Type::Constructor {..}) => (), // FIXME, error unexpected
            _ => (), // error type mismatch
        }
    }
    map
}

fn substitute(v: Vec<&mut Type>, map: HashMap<u16, Type>) {
    v.into_iter().map(|t| t.substitute_type(map));
}