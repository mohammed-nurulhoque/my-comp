use ast::{
    Expr as E1,
    Pattern,
};
use imper_ast::{
    Expr as E2,
    ValPath,
};
use types::{
    ProtoType,
    Type,
    Literal
};

use std::{
    ops::Deref,
    collections::HashMap,
    rc::Rc,
    cell::{
        RefCell,
        Ref,
    },
};

type type_constraint = (Rc<Type>, Rc<Type>);

enum NameInfo<'a> {
    Direct(&'a (ValPath, Rc<Type>)),
    Wrapped(Ref<'a, (ValPath, Rc<Type>)>),
}

impl<'a> Deref for NameInfo<'a> {
    type target = (ValPath, Rc<Type>);

    fn deref(&self) -> &Self::target {
        match *self {
            NameInfo::Direct(reference) => &reference,
            NameInfo::Wrapped(reference) => &reference,
        }
    }
}

pub struct Scope<'a> {
    local: HashMap<String, (ValPath, Rc<Type>)>,
    captures: RefCell<HashMap<&'a str, (usize, (ValPath, Rc<Type>))>>,
    next: Option<&'a Scope<'a>>,
}

impl<'a,'b> Scope<'a> where 'b: 'a {
    fn insert(&mut self, name: String, path: ValPath, t: &Rc<Type>) {
        self.local.insert(name, (path, Rc::clone(t)));
    }

     fn get(&self, name: &'b str) -> Option<NameInfo> {
        if let Some(val) = self.local.get(name) {
            return Some(NameInfo::Direct(val));
        } else {
            let map = self.captures.borrow();
            if let Some(_) =  map.get(name) {
                return Some(NameInfo::Wrapped(Ref::map(map, |map| &map.get(name).unwrap().1)));
            }
        }

        let mut node = self.next;
        let mut v = Vec::new();
        let (path, t): (ValPath, Rc<Type>) = loop {
            if let None = node {
                return None;
            }
            let val = node.unwrap();
            if let Some((path, t)) = val.local.get(name) {
                break (path.clone(), Rc::clone(t));
            } else if let Some((_, (path, t))) = val.captures.borrow().get(&name) {
                break (path.clone(), Rc::clone(t));
            }
            v.push(&val.captures);
            node = val.next;
        };

        v.into_iter().map(|x| (x.borrow_mut(), x.borrow().len())).rev()
            .fold(path, |path, (mut hm, len)| {
                hm.insert(name, (len, (path, Rc::clone(&t))));
                ValPath::Capture(vec![len])
            });
        self.get(name)
    }
}

enum Error {
    MultBindPattern(String),
    ConstructorNotFound(String),
}

impl Pattern {
    fn transform<'a> (self, 
        var: usize, next: usize,
        path: &mut Vec<usize>,
        map: &mut HashMap<String, (ValPath, Rc<Type>)>,
        scope: &Scope<'a>,
        type_consts: &mut Vec<type_constraint>, 
        val_consts: &mut HashMap<ValPath, Literal>
    ) -> Result<(), Error> {
        match self {
            Pattern::Wild => Ok(()),
            Pattern::Literal(Literal::Unit) => Ok(()),
            Pattern::Literal(l) => {
                let t = match l {
                    Literal::Int(_)     => Type::Int,
                    Literal::Bool(_)    => Type::Bool,
                    Literal::String(_)  => Type::String,
                };
                type_consts.push((Rc::new(Type::Variable(var)), Rc::new(t)));
                val_consts.insert(path.clone(), l);
                Ok(())
            },
            Pattern::Bind(s) => {
                match map.get(s) {
                    Some(_) => Err(Error::MultBindPattern(s)),
                    None => {
                        map.insert(s, (ValPath::Local(path.clone()), Rc::new(Type::Variable(var))));
                        Ok(())
                    }
                }
            },
            Pattern::Tuple(v) => {
                let len = v.len();
                type_consts.push((
                    Rc::new(Type::Variable(var)), 
                    Rc::new(Type::Tuple((0..len).map(|i| Type::Variable(next + i)).collect()))
                ));
                v.into_iter().enumerate().map(|(i, pat)| {
                    path.push(i);
                    let result = pat.transform(next + i, next + len, path, map,
                        scope, type_consts, val_consts);
                    path.pop();
                    result
                });
                Ok(()) // FIXME
            },
            Pattern::SumVar(constructor, pat) => {
                match scope.get(constructor) {
                    None => Err(Error::ConstructorNotFound(constructor)),
                    Some((_, t)) => { // fix <- not any type just constructor type
                        type_consts.push((Rc::new(Type::Variable(var)), Rc::clone(t)));
                        type_consts
                        Ok(()) // FIXME
                    }
                }
            },
        }
    }
}
/*
transform <- patterns in reverse
Vec<Pattern> * Dtree * constrained set * int * Scope * Vec<int> * int * mut
                        in dtree       ith pat         generic for each / next
-> Dtree * constrained set * int
*/
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