use ast::{
    Expr,
    Pattern,
};
use imper_ast::{
    Expr as iExpr,
    Closure,
    ValPath,
    TypeDecl,
};
use dtree::{
    WrappedTree,
};
use types::{
    BinOpcode,
    UnOpcode,
    ProtoType,
    Type,
    Literal
};
use std::{
    mem,
    cmp::max,
    ops::Deref,
    collections::{
        HashMap,
        BTreeMap,
    },
    cell::{
        RefCell,
        Ref,
    },
};

type TypeConstraint = (Type, Type);

/// A reference to the info about a name in a scope
pub enum NameInfo<'a> {
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

pub struct Scope<'a,'b> where 'a: 'b {
    /// the local variables (arguments)
    local: HashMap<String, (ValPath, Type)>,
    /// captures from an outer scopes
    captures: RefCell<HashMap<&'a str, (u16, (ValPath, Type))>>,
    /// reference to the next enclosing scope
    next: Option<&'b Scope<'a,'b>>,
}

/// A pattern is a set of constraints on a value, which are categorized as follows
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ConstraintValue {
    /// nth option out of x finitely many option, includes Booleans and union tags
    Finite(u16, u16),
    /// integer constraint which is technically finite but represented sparsely, so
    /// is practically inifinite
    Int(isize),
    /// string constraint, we allow strings in pattern matching
    Str(String),
}

impl<'a,'b> Scope<'a,'b> {
    /// insert a local name
    pub fn insert(&mut self, name: String, path: Vec<u16>, t: Type) {
        self.local.insert(name, (ValPath::Local(path), t));
    }

    /// get name from scope, if name not found in the current scope
    /// fills captures in all parent scopes until the containing scope
    pub fn get(&self, name: &str) -> Option<NameInfo> {
        if let Some(val) = self.local.get(name) {
            return Some(NameInfo::Direct(val));
        } else {
            let map = self.captures.borrow();
            if map.get(name).is_some() {
                return Some(NameInfo::Wrapped(Ref::map(map, |map| &map.get(name).unwrap().1)));
            }
        }

        let mut node: Option<&'b Scope<'a,'b>> = self.next;
        let mut v = vec![&self.captures];
        let (key, (path, t)): (&'a str, (ValPath, Type)) = loop {
            if node.is_none() {
                return None;
            }
            let scope = node.unwrap() as *const Scope<'a,'b>;
            unsafe {
                if let Some((key, val)) = (*scope).local.get_key_value(name) {
                    use self::ValPath::*;
                    match val.0 {
                        Local(_) => break (key, val.clone()),
                        Capture(_) => panic!("captured in local scope"),
                        StaticVal(_) | StaticFn(_) | Constructor => return Some(NameInfo::Direct(val)),
                    }
                } else {
                    if let Some((key, (n, (_, t)))) = (*scope).captures.borrow().get_key_value(name) {
                        break (key, (ValPath::Capture(vec![*n]), t.clone()));
                    }
                }
                v.push(&(*scope).captures);
                node = (*scope).next;
            }
        };

        v.into_iter().rev().map(|x| (x.borrow_mut(), x.borrow().len() as u16))
            .fold(path, |path, (mut hm, len)| {
                hm.insert(key, (len, (path, t.clone())));
                ValPath::Capture(vec![len])
            });
        self.get(name)
    }
}

/// instantiate a type by substituting generics with variables
/// starting from var, Generic(n) => Variable(var + n)
/// # RETURNS 
/// instantiated type and next free variable
fn gen2var(t: &Type, var: u16) -> (Type, u16) {
    match *t {
        Type::Unit | Type::Int | Type::Bool | Type::String => (t.clone(), var),
        Type::Constructor {ref arg, target, position} => {
            let (arg, next) = gen2var(arg, var);
            (Type::Constructor { arg: Box::new(arg), target, position }, next)
        },
        Type::Function(ref from, ref to) => {
            let (from, next) = gen2var(from, var);
            let (to, nnext) = gen2var(to, var);
            (Type::Function(Box::new(from), Box::new(to)), max(next, nnext))
        },
        Type::Generic(n) => (Type::Variable(var + n), var + n + 1),
        Type::Sum(n, ref t) => {
            let (t, next) = gen2var(&*t, var);
            (Type::Sum(n, Box::new(t)), next)
        },
        Type::Tuple(ref v) => {
            let (v, n): (Vec<Type>, Vec<u16>) = v.iter().map(|e| gen2var(e, var)).unzip();
            let n = n.into_iter().fold(var, |acc, elem| max(acc, elem));
            (Type::Tuple(v), n)
        },
        Type::Variable(_) => panic!("tried to instantiate instantiated type"),
    }
}

pub fn to_type(t: ProtoType,
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
            None     => panic!("Error, generic not found"), // to be removed
        },
        P::Sum(name, t)         => match map.get(&name) {
            Some(&n) => T::Sum(n, Box::new(to_type(*t, map, conver))),
            None     => panic!("should be error type not defined"), // to be removed
        },
    }
}

pub fn get_type_decl(
    name: String, vars: Vec<String>, variants: Vec<(String, ProtoType)>, 
    type_map: &mut HashMap<String, u16>, 
    val_map: &mut HashMap<String, (ValPath, Type)>,
) -> TypeDecl {
    let conver: HashMap<String, u16> = vars.into_iter().enumerate().map(|(i, s)| (s, i as u16)).collect();
    let len = type_map.len() as u16;
    type_map.insert(name.clone(), len);
    TypeDecl {
        name, num_generics: conver.len() as u16, 
        variants: variants.into_iter().enumerate().map(|(i, (s, t))| {
            let t = to_type(t, type_map, &conver);
            val_map.insert(s.clone(), (ValPath::Constructor, Type::Constructor {
                arg: Box::new(t.clone()), target: len, position: (i + 1) as u16
            }));
            (s, t)
        }).collect()
    }
}

pub enum Error {
    NameNotFound(String),
    MultBindPattern(String),
    ConstructorNotFound(String),
    NonConstAppPattern(String),
    TypeNotDefined(String),
    VariablePatsNum,
}

impl Literal {
    fn get_constraint(self) -> ConstraintValue {
        match self {
            Literal::Unit       => panic!("trying to get constraint from unit"),
            Literal::Int(n)     => ConstraintValue::Int(n),
            Literal::Bool(true) => ConstraintValue::Finite(0,2),
            Literal::Bool(false)=> ConstraintValue::Finite(1,2),
            Literal::String(s)  => ConstraintValue::Str(s),
        }
    }
}

impl Pattern {
    pub fn transform<'a,'b> (self, 
        var: u16, next: u16,
        path: &mut Vec<u16>,
        type_map: &Vec<TypeDecl>,
        local: &mut HashMap<String, (ValPath, Type)>,
        scope: &Scope<'a,'b>,
        type_consts: &mut Vec<TypeConstraint>, 
        val_consts: &mut BTreeMap<ValPath, ConstraintValue>,
        errors: &mut Vec<Error>) -> u16 {
        match self {
            Pattern::Wild => next,
            Pattern::Literal(l) => {
                type_consts.push((Type::Variable(var), l.get_type()));
                if let Literal::Unit = l { () }
                else {
                    val_consts.insert(ValPath::Local(path.clone()), l.get_constraint());
                }
                next
            },
            Pattern::Bind(s) => {
                match scope.local.get(&s) {
                    Some(_) => { errors.push(Error::MultBindPattern(s)); next },
                    None => {
                        local.insert(s, (ValPath::Local(path.clone()), Type::Variable(var)));
                        next
                    },
                }
            },
            Pattern::Tuple(v) => {
                let len = v.len() as u16;
                let mut nnext = next + len;
                type_consts.push((
                    Type::Variable(var), 
                    Type::Tuple((next..nnext).map(|i| Type::Variable(i)).collect())
                ));
                for (i, pat) in v.into_iter().enumerate() {
                    let i = i as u16;
                    path.push(i);
                    nnext = pat.transform(next + i, nnext, path, type_map,
                        local, scope, type_consts, val_consts, errors);
                    path.pop();
                }
                nnext
            },
            Pattern::SumVar(constructor, pat) => match scope.get(&constructor) {
                None => { errors.push(Error::ConstructorNotFound(constructor)); next },
                Some(ni) => if let Type::Constructor { ref arg, target, position } = ni.1 {

                        //val_Consts
                        path.push(0);
                        let t = &type_map[target as usize];
                        val_consts.insert(ValPath::Local(path.clone()), 
                            ConstraintValue::Finite(position, t.variants.len() as u16));

                        let (from, n1) = gen2var(arg, next + 1);
                        let (to, n2) = (Type::Sum(target, 
                            Box::new(Type::Tuple((0..t.num_generics).map(|n| Type::Variable(next + 1 + n)).collect()))),
                            next + 1 + t.num_generics);
                        type_consts.push((Type::Variable(var), to));
                        type_consts.push((Type::Variable(next), from));
                        path.push(position);
                        let next = pat.transform(next, max(n1, n2), path, type_map, 
                            local, scope, type_consts, val_consts, errors);
                        path.pop();
                        next
                } else {
                    errors.push(Error::NonConstAppPattern(constructor));
                    next
                }
            },
        }
    }
}

impl Expr {
    pub fn transform<'a,'b>(self,
        var: u16, next: u16,
        type_map: &Vec<TypeDecl>,
        type_refs: &mut Vec<*mut Type>,
        scope: &'b Scope<'a,'b>,
        type_consts: &mut Vec<TypeConstraint>,
        errors: &mut Vec<Error>,
    ) -> (iExpr, u16) {
        let sequence = |e1: Expr, e2: Expr, var1, var2, next,
            type_refs: &mut Vec<*mut Type>,
            type_consts: &mut Vec<TypeConstraint>,
            errors: &mut Vec<Error>| 
        {
            let (e1, next) = e1.transform(var1, next, type_map, type_refs, scope, type_consts, errors);
            let (e2, next) = e2.transform(var2, next, type_map, type_refs, scope, type_consts, errors);
            (e1, e2, next)
        };
        match self {
            Expr::Literal(l) => {
                type_consts.push((Type::Variable(var), l.get_type()));
                (iExpr::Literal(l), next)
            },
            Expr::Bound(s) => match scope.get(&s) {
                Some(ni) => {
                    let (path , t) = &* ni;
                    let mut t = t.clone();
                    type_refs.push(&mut t);
                    (iExpr::Bound(path.clone(), t), next)
                },
                None => { errors.push(Error::NameNotFound(s)); (iExpr::Error, next) },
            },
            Expr::BinOp(e1, op, e2) => {
                use self::BinOpcode::*;
                let (e1, e2, next) = match op {
                    Add | Sub | Mul | Div | Mod => {
                        type_consts.push((Type::Variable(var), Type::Int));
                        sequence(*e1, *e2, var, var, next,type_refs,  type_consts, errors)
                    },
                    Greater | Less | GreaterEq | LessEq => {
                        type_consts.push((Type::Variable(var), Type::Bool));
                        type_consts.push((Type::Variable(next), Type::Int));
                        sequence(*e1, *e2, next, next, next + 1,type_refs, type_consts, errors)
                    },
                    Equal | NotEq => {
                        type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, next, next, next + 1, type_refs, type_consts, errors)
                    },
                    And | Or => {
                        type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, var, var, next, type_refs, type_consts, errors)
                    }
                };
                (iExpr::BinOp(Box::new(e1), op, Box::new(e2)), next)
            },
            Expr::UnOp(UnOpcode::Minus, e) => {
                type_consts.push((Type::Variable(var), Type::Int));
                let (e, next) = e.transform(var, next, type_map, type_refs, scope, type_consts, errors);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            },
            Expr::UnOp(UnOpcode::Not, e) => {
                type_consts.push((Type::Variable(var), Type::Bool));
                let (e, next) = e.transform(var, next, type_map, type_refs, scope, type_consts, errors);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            },
            Expr::Tuple(v) => {
                let mut v2 = Vec::new();
                let mut nnext = next + v.len() as u16;
                type_consts.push((Type::Variable(var), 
                    Type::Tuple((0..v.len()).map(|i| Type::Variable(next + i as u16)).collect())));
                for (i, e) in v.into_iter().enumerate() {
                    let (e, next) = e.transform(next + i as u16, nnext, type_map, type_refs, scope, type_consts, errors);
                    v2.push(e);
                    nnext = next;
                }
                (iExpr::Tuple(v2), nnext)
            },
            Expr::Application(e1, e2) => {
                type_consts.push((Type::Variable(next), 
                    Type::Function(Box::new(Type::Variable(next + 1)),
                    Box::new(Type::Variable(var)))));
                let (e1, e2, next) = sequence(*e1, *e2, next, next, next + 1, type_refs, type_consts, errors);
                (iExpr::Application(Box::new(e1), Box::new(e2)), next)
            },
            Expr::Conditional(cond, e1, e2) => {
                type_consts.push((Type::Variable(next), Type::Bool));
                let (cond, next) = cond.transform(next, next + 1, type_map, type_refs, scope, type_consts, errors);
                let (e1, e2, next) = sequence(*e1, *e2, var, var, next, type_refs, type_consts, errors);
                (iExpr::Conditional(Box::new(cond), Box::new(e1), Box::new(e2)), next)
            },
            Expr::Function(v) => {
                let (closure, next) = fn_transform(v, next, next+1, type_map, type_refs, scope, type_consts, errors);
                (iExpr::Closure(closure), next)
            },
        }
    }
}

/// # REQUIRES
/// count > 0
fn mk_curried_type(from: u16, count: u16) -> Type {
    let mut t = Type::Variable(from + count - 1);
    for i in (from..(count-1)).rev() {
        t = Type::Function(Box::new(Type::Variable(i)), Box::new(t));
    }
    t
}

fn fn_transform<'a,'b>(
    fn_branches: Vec<(Vec<Pattern>, Expr)>,
    var: u16, next: u16,
    type_map: &Vec<TypeDecl>,
    type_refs: &mut Vec<*mut Type>,
    scope: &'b Scope<'a,'b>,
    type_consts: &mut Vec<TypeConstraint>,
    errors: &mut Vec<Error>,
) -> (Closure, u16) {
    let len = fn_branches[0].0.len() as u16;
    debug_assert!(len > 0);
    type_consts.push((Type::Variable(var), mk_curried_type(next, len + 1)));
    let mut nnext = next + len + 1;
    let mut wrapped = WrappedTree::new();
    let mut branches = Vec::new();
    let mut captures = RefCell::new(HashMap::new());
    for (i, (pats, e)) in fn_branches.into_iter().enumerate().rev() {
        if pats.len() as u16 != len {
            errors.push(Error::VariablePatsNum);
        }

        let mut path = vec![];
        let mut val_consts = BTreeMap::new();
        let mut local = HashMap::new();
        for (j, pat) in pats.into_iter().enumerate() {
            path.push(j as u16);
            pat.transform(next + j as u16, nnext, &mut path, type_map, &mut local, scope, type_consts, &mut val_consts, errors);
            path.pop();
        }
        wrapped.add_pattern(val_consts, i as u16);
        let mut scope = Scope {
            local,
            captures,
            next: Some(scope),
        };
        let (e, tmp) = e.transform(next + len, nnext, type_map, type_refs, &scope, type_consts, errors);
        branches.push(e);
        nnext = tmp;
        captures = RefCell::new(HashMap::new());
        mem::swap(&mut captures, &mut scope.captures);
    }

    let mut captures: Vec<_> = captures.into_inner().into_iter().map(|(_, value)| value).collect();
    captures.sort_unstable_by(|(ord1, _), (ord2, _)| ord1.cmp(ord2));
    let captures: Vec<(ValPath, Type)> = captures.into_iter().map(|(_, v)| v).collect();

    let mut result = Closure {
        captures, dtree: wrapped.dtree, branches,
        args: (next..(next + len)).map(|n| Type::Variable(n)).collect(),
        return_type: Type::Variable(next + len),
    };
    type_refs.push(&mut result.return_type);
    for t in &mut result.args {
        type_refs.push(t);
    }

    (result, nnext)
}