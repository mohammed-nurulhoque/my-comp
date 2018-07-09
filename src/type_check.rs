use ast::{
    Expr as E1
};
use imper_ast::Expr;
use types::{ProtoType, Type};
use std::{
    collections::HashMap,
    rc::Rc,
};

/*
struct Map<'a> {
    map: HashMap<String, Item<'a>>,
    next: usize,
}

enum Item<'a> {
    Name(Type, usize),
    Namespace(&'a Map<'a>, NSType),
}

struct Scope<'a> {
    map: Map<'a>,
    parent: Option<&'a Scope<'a>>,
}

#[derive(Clone, Copy, PartialEq)]
enum NSType { Module, Union }

struct Constraint {
    right:  Rc<Type>,
    left:   Rc<Type>,
}

impl<'a> Map<'a> {
    fn insert(&mut self, name: String, t: Type) -> (usize, Option<Item>) {
        let prev = self.map.insert(name, Item::Name(t, self.next));
        self.next += 1;
        (self.next-1, prev)
    }

    fn get(&self, names: &[String]) -> Option<(&Type, usize, NSType)> {
        use self::Item::*;
        let mut names = names;
        let mut ns = &self.map;
        let mut nstype = NSType::Module;
        loop {
            match ns.get(&names[0]) {
                None => return None,
                Some(Name (t, n)) => return Some((t,*n, nstype)),
                Some(Namespace(ns_, nstype_)) => {
                    if *nstype_ == NSType::Union {
                        nstype = *nstype_;
                    }
                    names = &names[1..];
                    ns = &ns_.map;
                },
            }
        }
    }
}

impl<'a> Scope<'a> {
    fn get(&self, names: &[String]) -> Option<(&Type, usize, NSType)> {
        let result = self.map.get(names);
        match result {
            Some(_) => result,
            None => {
                if let Some(ns) = self.parent { ns.get(names) } 
                else { None }
            },
        }
    }
}

impl<'a,'b> P1 where 'b: 'a {
    fn transform(
        self, 
        names: &'a mut Map<'b>, 
        consts: &mut Vec<Constraint>, 
        parent: usize, next: usize
    ) -> Result<(P2,usize), Vec<String>>
    {
        use self::Type::{self as T2, Generic};
        match self {
            P1::Literal(l) => {
                consts.push(Constraint { left: Rc::new(Generic(parent)), right: Rc::new(l.get_type()) });
                Ok((P2::Literal(l), next))
            },
            P1::Bind(s) => {
                let (n, _) = names.insert(s, Generic(parent));
                Ok((P2::Bind(n), next))
            },
            P1::Tuple(v) => {
                consts.push(Constraint { 
                    left: Rc::new(Generic(parent)),
                    right: Rc::new(T2::Tuple((0..v.len()).map(|n| Generic(next + n)).collect()))
                });
                let mut u = Vec::new();
                let len = v.len();
                let next = v.into_iter()
                    .enumerate()
                    .fold(Ok(next + len), |acc: Result<usize,Vec<String>>, (n, pattern)| {
                        let (e, newnext) = pattern.transform(names, consts, next + n, acc?)?;
                        u.push(e);
                        Ok(newnext)
                    })?;
                Ok((P2::Tuple(u), next))
            },
            P1::Wild => Ok((P2::Wild, next)),
            P1::SumVar(path, pattern) => {
                let mut type_n;
                if let Some((&T2::Function(ref from, ref to), n, NSType::Union)) = names.get(&path[..]) {
                    type_n = n;
                    consts.push(Constraint { left: Rc::new(Generic(parent)), right: Rc::clone(to) });
                    consts.push(Constraint { left: Rc::new(Generic(next)), right: Rc::clone(from) });
                } else {
                    return Err(path)
                }
                let (pat, next) = (*pattern).transform(names, consts, next, next + 1)?;
                Ok((P2::SumVar(type_n, Box::new(pat)), next))
            },
        }
    }
}

impl E1 {
    fn collect_constraints<'a>(&self,
        names: &Scope<'a>,
        consts: &mut Vec<Constraint>,
        parent: usize, next: usize
    ) -> (usize)
    {
        match *self {
            _ => 0,
        }
    }
}
*/