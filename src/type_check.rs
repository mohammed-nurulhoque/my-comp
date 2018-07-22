use ast::{
    Expr as E1,
    Pattern,
};
use imper_ast::{
    Expr as E2,
    ValPath,
    TypeDecl,
};
use types::{
    ProtoType,
    Type,
    Literal
};
use std::{
    cmp::max,
    ops::Deref,
    collections::HashMap,
    cell::{
        RefCell,
        Ref,
    },
};

type type_constraint = (Type, Type);

enum NameInfo<'a> {
    Direct(&'a (ValPath, Type)),
    Wrapped(Ref<'a, (ValPath, Type)>),
}

impl<'a> Deref for NameInfo<'a> {
    type Target = (ValPath, Type);

    fn deref(&self) -> &Self::Target {
        match *self {
            NameInfo::Direct(reference) => reference,
            NameInfo::Wrapped(ref reference) => &*reference,
        }
    }
}

pub struct Scope<'a> {
    local: HashMap<String, (ValPath, Type)>,
    captures: RefCell<HashMap<&'a str, (u16, (ValPath, Type))>>,
    next: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    fn insert(&mut self, name: String, path: Vec<u16>, t: Type) {
        self.local.insert(name, (ValPath::Local(path), t));
    }

    fn get(&'a self, name: &str) -> Option<NameInfo> {
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
        let (key, (path, t)): (&'a str, (ValPath, Type)) = loop {
            if let None = node {
                return None;
            }
            let val = node.unwrap();
            if let Some((key, (path, t))) = val.local.get_key_value(name) {
                break (key, (path.clone(), t.clone()));
            } else {
                if let Some((key, (n, (_, t)))) = val.captures.borrow().get_key_value(name) {
                    break (key, (ValPath::Capture(vec![*n]), t.clone()));
                }
            }
            v.push(&val.captures);
            node = val.next;
        };

        v.into_iter().map(|x| (x.borrow_mut(), x.borrow().len() as u16)).rev()
            .fold(path, |path, (mut hm, len)| {
                hm.insert(key, (len, (path, t.clone())));
                ValPath::Capture(vec![len])
            });
        self.get(name)
    }
}

fn gen2var(t: &Type, var: u16) -> (Type, u16) {
    match *t {
        Type::Unit | Type::Int | Type::Bool | Type::String => (t.clone(), var),
        Type::Constructor(ref t, n) => {
            let (t, next) = gen2var(t, var);
            (Type::Constructor(Box::new(t), n), next)
        },
        Type::Function(ref from, ref to) => {
            let (from, n1) = gen2var(from, var);
            let (to, n2) = gen2var(to, var);
            (Type::Function(Box::new(from), Box::new(to)), max(n1,n2))
        },
        Type::Generic(n) => (Type::Variable(var + n), var + n + 1),
        Type::Sum(n, ref t) => {
            let (t, n) = gen2var(&*t, var);
            (Type::Sum(n, Box::new(t)), n)
        },
        Type::Tuple(ref v) => {
            let (v, n): (Vec<Type>, Vec<u16>) = v.into_iter().map(|e| gen2var(e, var)).unzip();
            let n = n.into_iter().fold(var, |acc, elem| max(acc, elem));
            (Type::Tuple(v), n)
        },
        Type::Variable(n) => (t.clone(), var),
    }
}

fn to_type(t: ProtoType,
    map: &HashMap<String, u16>, // map of type names -> index in types vector
    conver: &HashMap<String, u16> // map of generics to variables
) -> Type {
    use self::ProtoType as P;
    use self::Type as T;
    match t {
        P::Unit                 => T::Unit,
        P::Int                  => T::Int,
        P::Bool                 => T::Bool,
        P::String               => T::String,
        P::Tuple(v)             => T::Tuple(v.into_iter().map(|t| to_type(t, map, conver)).collect()),
        P::Function(from, to)   => T::Function(Box::new(to_type(*from, map, conver)), 
                                               Box::new(to_type(*to, map, conver))),
        P::Generic(name)        => match conver.get(&name) {
            Some(&n) => T::Generic(n),
            None     => panic!("Error, generic not found"),
        },
        P::Sum(name, t)         => match map.get(&name) {
            Some(&n) => T::Sum(n, Box::new(to_type(*t, map, conver))),
            None     => panic!("should be error type not defined"),
        },
    }
}

fn get_type_decl(
    name: String, vars: Vec<String>, variants: Vec<(String, ProtoType)>, 
    map: &mut HashMap<String, u16>) -> TypeDecl {
    let conver: HashMap<String, u16> = vars.into_iter().enumerate().map(|(i, s)| (s, i as u16)).collect();
    let len = map.len() as u16;
    map.insert(name.clone(), len);
    let result = TypeDecl {
        name, num_generics: conver.len() as u16, 
        variants: variants.into_iter().map(|(s, t)| (s, to_type(t, map, &conver))).collect()
    };
    result
}

enum Error {
    MultBindPattern(String),
    ConstructorNotFound(String),
    NonConstAppPattern(String),
    TypeNotDefined(String),
}

impl Pattern {
    fn transform<'a> (self, 
        var: u16, next: u16,
        path: &mut Vec<u16>,
        val_map: &mut HashMap<String, (ValPath, Type)>,
        type_map: &Vec<TypeDecl>,
        scope: &'a Scope<'a>,
        type_consts: &mut Vec<type_constraint>, 
        val_consts: &mut HashMap<ValPath, Literal>,
        errors: Vec<Error>) {
        match self {
            Pattern::Wild => (),
            Pattern::Literal(l) => {
                let t = match l {
                    Literal::Unit       => Type::Unit,
                    Literal::Int(_)     => Type::Int,
                    Literal::Bool(_)    => Type::Bool,
                    Literal::String(_)  => Type::String,
                };
                type_consts.push((Type::Variable(var), t));
                val_consts.insert(ValPath::Local(path.clone()), l);
            },
            Pattern::Bind(s) => {
                match val_map.get(&s) {
                    Some(_) => errors.push(Error::MultBindPattern(s)),
                    None => {val_map.insert(s, (ValPath::Local(path.clone()), Type::Variable(var)));},
                }
            },
            Pattern::Tuple(v) => {
                let len = v.len() as u16;
                type_consts.push((
                    Type::Variable(var), 
                    Type::Tuple((0..len).map(|i| Type::Variable(next + i)).collect())
                ));
                v.into_iter().enumerate().map(|(i, pat)| {
                    let i = i as u16;
                    path.push(i);
                    let result = pat.transform(next + i, next + len, path, val_map, type_map,
                        scope, type_consts, val_consts,errors);
                    path.pop();
                    result
                });
            },
            Pattern::SumVar(constructor, pat) => match scope.get(&constructor) {
                None => {errors.push(Error::ConstructorNotFound(constructor));},
                Some(ni) => if let Type::Constructor(ref t, n) = ni.1 {
                        let (from, n1) = gen2var(t, next + 1);
                        let (to, n2) = gen2var(&Type::Sum(n, (0..type_map[n as usize].num_generics).map(|n| Type::Generic(n)).collect()), next + 1);
                        type_consts.push((Type::Variable(var), to));
                        type_consts.push((Type::Variable(next), from));
                        // path.push() // FIX, put order of const in type
                        pat.transform(next, max(n1, n2), path, val_map, type_map, scope, type_consts, val_consts, errors);
                        path.pop();
                } else {
                    errors.push(Error::NonConstAppPattern(constructor));
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