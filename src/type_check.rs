//! Author: Mohammed Nurul Hoque (2018)
//! 
//! This module contains the logic for transforming a compilation unit from AST
//! to imperAST.
use ast::{Binding, Expr, Pattern};
use dtree::DTree;
use imper_ast::{Closure, ConstraintValue, Expr as iExpr, Module, TypeDecl, ValPath};
use std::{
    cell::{Ref, RefCell}, cmp::max, collections::{BTreeMap, HashMap}, mem, ops::Deref,
};
use types::{BinOpcode, Literal, ProtoType, Type, UnOpcode};
use unify;

/// All errors from AST -> imperAST phase, not used yet!
#[derive(Debug)]
pub enum Error {
    TypeMismatch(Type, Type),
    ConstructorUnification,
    NameNotFound(String),
    MultBindPattern(String),
    ConstructorNotFound(String),
    NonConstAppPattern(String),
    TypeNotDefined(String),
    VariablePatsNum,
}

/// The transformation function, takes a series of bindings in AST form,
/// which are either value binding or type declarations. Converts to
/// a Module struct (see imper_ast.rs) which separated functions and
/// variables.
pub fn ast2imper_ast(bindings: Vec<Binding>) -> Result<Module, Error> {
    let mut errors = Vec::new();
    let mut type_map = HashMap::new();
    let mut static_funcs = Vec::new();
    let mut anon_funcs = Vec::new();
    let global_names = HashMap::new();
    let mut globals = Vec::new();
    let mut type_decls = Vec::new();
    let captures = RefCell::new(HashMap::new());
    let mut val_order = 0;
    let mut fun_order = 0;

    let mut scope = Scope {
        local: global_names,
        captures,
        next: None,
    };
    for binding in bindings {
        match binding {
            Binding::Function { name, variants } => {
                static_funcs.push((
                    static_fn_transform(
                        &name,
                        fun_order,
                        variants,
                        &mut anon_funcs,
                        &mut type_decls,
                        &mut scope,
                        &mut errors,
                    )?,
                    name,
                ));
                fun_order += 1;
            }
            Binding::Type { name, vars, variants,} => type_decls.push(
                get_type_decl(name, vars, variants, &mut type_map, &mut scope,
            )),
            Binding::Value(pat, expr) => {
                globals.push(binding_transform(
                    val_order,
                    pat,
                    expr,
                    &mut anon_funcs,
                    &mut type_decls,
                    &mut scope,
                    &mut errors,
                )?);
                val_order += 1;
            }
        }
    }

    Ok(Module {
        static_funcs, anon_funcs, globals, type_decls,
        globals_names: scope.local.into_iter()
            .map(|(s, (path, _))| (s, path)).collect(),
    })
}

type TypeConstraint = (Type, Type);

/// wraps arguments for conciseness
struct Args<'a, 'b, 'c, 'd, 'e>
where
    'b: 'c,
{
    type_decls: &'a Vec<TypeDecl>,
    scope: &'c Scope<'b, 'c>,
    type_consts: &'d mut Vec<TypeConstraint>,
    errors: &'e mut Vec<Error>,
}

/// A reference to the info about a name in a scope,
/// this is required because we use a refcell for captured values which
/// returns "Ref"s, the reference inside cannot be moved out of the Ref.
enum NameInfo<'a> {
    Direct(&'a (ValPath, Type)),
    DTree(Ref<'a, (ValPath, Type)>),
}

impl<'a> Deref for NameInfo<'a> {
    type Target = (ValPath, Type);

    fn deref(&self) -> &Self::Target {
        match *self {
            NameInfo::Direct(reference) => reference,
            NameInfo::DTree(ref reference) => &*reference,
        }
    }
}

struct Scope<'a, 'b>
where
    'a: 'b,
{
    /// the local variables (arguments)
    local: HashMap<String, (ValPath, Type)>,
    /// captures from an outer scopes
    captures: RefCell<HashMap<&'a str, (u16, (ValPath, Type))>>,
    /// reference to the next enclosing scope
    next: Option<&'b Scope<'a, 'b>>,
}

impl<'a, 'b> Scope<'a, 'b> {
    /// get name from scope, if name not found in the current scope
    /// fills captures in all parent scopes until the containing scope
    fn get(&self, name: &str) -> Option<NameInfo> {
        if let Some(val) = self.local.get(name) {
            return Some(NameInfo::Direct(val));
        } else {
            let map = self.captures.borrow();
            if map.get(name).is_some() {
                return Some(NameInfo::DTree(Ref::map(map, |map| {
                    &map.get(name).unwrap().1
                })));
            }
        }

        let mut node: Option<&'b Scope<'a, 'b>> = self.next;
        let mut v = vec![&self.captures];
        let (key, (path, t)): (&'a str, (ValPath, Type)) = loop {
            if node.is_none() {
                return None;
            }
            let scope = node.unwrap() as *const Scope<'a, 'b>;
            unsafe {
                if let Some((key, val)) = (*scope).local.get_key_value(name) {
                    use self::ValPath::*;
                    match val.0 {
                        Local(_) => break (key, val.clone()),
                        Capture(_) => panic!("captured in local scope"),
                        StaticVal(_) | StaticFn(_) | Constructor => {
                            return Some(NameInfo::Direct(val))
                        }
                    }
                } else {
                    if let Some((key, (n, (_, t)))) = (*scope).captures.borrow().get_key_value(name)
                    {
                        break (key, (ValPath::Capture(vec![*n]), t.clone()));
                    }
                }
                v.push(&(*scope).captures);
                node = (*scope).next;
            }
        };

        v.into_iter()
            .rev()
            .map(|x| (x.borrow_mut(), x.borrow().len() as u16))
            .fold(path, |path, (mut hm, len)| {
                hm.insert(key, (len, (path, t.clone())));
                ValPath::Capture(vec![len])
            });
        self.get(name)
    }
}

/// This code is a real mess, needs refactoring
fn binding_transform<'a, 'b>(
    order: u16,
    pat: Pattern,
    expr: Expr,
    anon_funcs: &mut Vec<Closure>,
    type_decls: &Vec<TypeDecl>,
    scope: &mut Scope<'a, 'b>,
    errors: &mut Vec<Error>,
) -> Result<(iExpr, BTreeMap<ValPath, ConstraintValue>, Type), Error> {
    let mut path = vec![order];
    let mut local = HashMap::new();
    let mut _captures = RefCell::new(HashMap::new()); // discarded
    let mut type_consts = Vec::new();
    let mut val_consts = BTreeMap::new();
    let next = {
        let mut args = Args {
            type_decls,
            scope,
            type_consts: &mut type_consts,
            errors,
        };
        pat.transform(
            0,
            1,
            &mut path,
            &mut args,
            ValPath::StaticVal,
            &mut local,
            &mut val_consts,
        )
    };

    let mut anon_tmp = Vec::new();
    let (expr, local) = {
        let mut scope = Scope {
            local,
            captures: _captures,
            next: Some(scope),
        };
        let expr = {
            let mut args = Args {
                type_decls,
                scope: &scope,
                type_consts: &mut type_consts,
                errors,
            };
            expr.transform(0, next, &mut args, &mut anon_tmp).0
        };
        let mut hm = HashMap::new();
        mem::swap(&mut scope.local, &mut hm);
        (expr, hm)
    };

    let mut map = unify::unify(type_consts)?;
    for closure in &mut anon_tmp {
        closure.substitute_types(&map);
    }

    anon_funcs.extend(anon_tmp);
    scope.local.extend(local);

    let mut t = Type::Variable(0);
    t.substitute_type(&mut map);
    Ok((expr, val_consts, t))
}

/// transform a top-level function to imper_ast representaion,
/// top-level functions can be recursive
fn static_fn_transform<'a, 'b>(
    name: &str,
    order: u16,
    fn_branches: Vec<(Vec<Pattern>, Expr)>,
    anon_funcs: &mut Vec<Closure>,
    type_decls: &Vec<TypeDecl>,
    scope: &mut Scope<'a, 'b>,
    errors: &mut Vec<Error>,
) -> Result<Closure,Error> {
    scope.local.insert(
        name.to_string(),
        (ValPath::StaticFn(order), Type::Variable(0)),
    );
    let mut type_consts = Vec::new();
    let mut anon_tmp = Vec::new();
    let mut result = {
        let mut args = Args {
            type_decls,
            type_consts: &mut type_consts,
            scope,
            errors,
        };
        fn_transform(fn_branches, 0, 1, &mut args, &mut anon_tmp).0
    };

    let map = unify::unify(type_consts)?;
    result.substitute_types(&map);
    for closure in &mut anon_tmp {
        closure.substitute_types(&map);
    }
    anon_funcs.extend(anon_tmp);
    Ok(result)
}

fn get_type_decl<'a, 'b>(
    name: String,
    vars: Vec<String>,
    variants: Vec<(String, ProtoType)>,
    type_map: &mut HashMap<String, u16>,
    scope: &mut Scope<'a, 'b>,
) -> TypeDecl {
    let conver: HashMap<String, u16> = vars
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i as u16))
        .collect();
    let len = type_map.len() as u16;
    type_map.insert(name.clone(), len);
    TypeDecl {
        name,
        num_generics: conver.len() as u16,
        variants: variants
            .into_iter()
            .enumerate()
            .map(|(i, (s, t))| {
                let t = to_type(t, type_map, &conver);
                scope.local.insert(
                    s.clone(),
                    (
                        ValPath::Constructor,
                        Type::Constructor {
                            arg: Box::new(t.clone()),
                            target: len,
                            position: (i + 1) as u16,
                        },
                    ),
                );
                (s, t)
            })
            .collect(),
    }
}

fn fn_transform<'a, 'b, 'c, 'd, 'e>(
    fn_branches: Vec<(Vec<Pattern>, Expr)>,
    var: u16,
    next: u16,
    args: &mut Args<'a, 'b, 'c, 'd, 'e>,
    anon_funcs: &mut Vec<Closure>,
) -> (Closure, u16) {
    let len = fn_branches[0].0.len() as u16;
    debug_assert!(len > 0);
    args.type_consts
        .push((Type::Variable(var), mk_curried_type(next, len + 1)));
    let mut nnext = next + len + 1;
    let mut dtree = DTree::new();
    let mut branches = Vec::new();
    let mut captures = RefCell::new(HashMap::new());
    for (i, (pats, e)) in fn_branches.into_iter().enumerate().rev() {
        if pats.len() as u16 != len {
            args.errors.push(Error::VariablePatsNum);
        }

        let mut path = vec![];
        let mut val_consts = BTreeMap::new();
        let mut local = HashMap::new();
        for (j, pat) in pats.into_iter().enumerate() {
            path.push(j as u16);
            pat.transform(
                next + j as u16,
                nnext,
                &mut path,
                args,
                ValPath::Local,
                &mut local,
                &mut val_consts,
            );
            path.pop();
        }
        dtree.add_pattern(&mut val_consts, i as u16);
        let mut scope = Scope {
            local,
            captures,
            next: Some(args.scope),
        };
        unsafe {
            let mut reference: &'c Scope = mem::transmute::<&_, &'c _>(&scope);
            mem::swap(&mut reference, &mut args.scope);
            let (e, tmp) = e.transform(next + len, nnext, args, anon_funcs);
            mem::swap(&mut reference, &mut args.scope);
            branches.push(e);
            nnext = tmp;
        }
        captures = RefCell::new(HashMap::new());
        mem::swap(&mut captures, &mut scope.captures);
    }

    let mut captures: Vec<_> = captures
        .into_inner()
        .into_iter()
        .map(|(_, value)| value)
        .collect();
    captures.sort_unstable_by(|(ord1, _), (ord2, _)| ord1.cmp(ord2));
    let captures: Vec<(ValPath, Type)> = captures.into_iter().map(|(_, v)| v).collect();

    (
        Closure {
            captures,
            dtree,
            branches,
            args: (next..(next + len)).map(|n| Type::Variable(n)).collect(),
            return_type: Type::Variable(next + len),
        },
        nnext,
    )
}

impl Pattern {
    fn transform<'a, 'b, 'c, 'd, 'e, T: Fn(Vec<u16>) -> ValPath + Copy>(
        self,
        var: u16,
        next: u16,
        path: &mut Vec<u16>,
        args: &mut Args<'a, 'b, 'c, 'd, 'e>,
        fn2val_path: T,
        local: &mut HashMap<String, (ValPath, Type)>,
        val_consts: &mut BTreeMap<ValPath, ConstraintValue>,
    ) -> u16 {
        match self {
            Pattern::Wild => next,
            Pattern::Literal(l) => {
                args.type_consts.push((Type::Variable(var), l.get_type()));
                if let Literal::Unit = l {
                    ()
                } else {
                    val_consts.insert(fn2val_path(path.clone()), l.get_constraint());
                }
                next
            }
            Pattern::Bind(s) => match args.scope.local.get(&s) {
                Some(_) => {
                    args.errors.push(Error::MultBindPattern(s));
                    next
                }
                None => {
                    local.insert(s, (fn2val_path(path.clone()), Type::Variable(var)));
                    next
                }
            },
            Pattern::Tuple(v) => {
                let len = v.len() as u16;
                let mut nnext = next + len;
                args.type_consts.push((
                    Type::Variable(var),
                    Type::Tuple((next..nnext).map(|i| Type::Variable(i)).collect()),
                ));
                for (i, pat) in v.into_iter().enumerate() {
                    let i = i as u16;
                    path.push(i);
                    nnext =
                        pat.transform(next + i, nnext, path, args, fn2val_path, local, val_consts);
                    path.pop();
                }
                nnext
            }
            Pattern::SumVar(constructor, pat) => match args.scope.get(&constructor) {
                None => {
                    args.errors.push(Error::ConstructorNotFound(constructor));
                    next
                }
                Some(ni) => if let Type::Constructor {
                    ref arg,
                    target,
                    position,
                } = ni.1
                {
                    path.push(0);
                    let t = &args.type_decls[target as usize];
                    val_consts.insert(
                        fn2val_path(path.clone()),
                        ConstraintValue::Finite(position, t.variants.len() as u16),
                    );

                    let (from, n1) = gen2var(arg, next + 1);
                    let (to, n2) = (
                        Type::Sum(
                            target,
                            Box::new(Type::Tuple(
                                (0..t.num_generics)
                                    .map(|n| Type::Variable(next + 1 + n))
                                    .collect(),
                            )),
                        ),
                        next + 1 + t.num_generics,
                    );
                    args.type_consts.push((Type::Variable(var), to));
                    args.type_consts.push((Type::Variable(next), from));
                    path.push(position);
                    let next = pat.transform(
                        next,
                        max(n1, n2),
                        path,
                        args,
                        fn2val_path,
                        local,
                        val_consts,
                    );
                    path.pop();
                    next
                } else {
                    args.errors.push(Error::NonConstAppPattern(constructor));
                    next
                },
            },
        }
    }
}

impl Expr {
    fn transform<'a, 'b, 'c, 'd, 'e>(
        self,
        var: u16,
        next: u16,
        args: &mut Args<'a, 'b, 'c, 'd, 'e>,
        anon_funcs: &mut Vec<Closure>,
    ) -> (iExpr, u16) {
        let sequence = |e1: Expr,
                        e2: Expr,
                        var1,
                        var2,
                        next,
                        args: &mut Args<'a, 'b, 'c, 'd, 'e>,
                        anon_funcs: &mut Vec<Closure>| {
            let (e1, next) = e1.transform(var1, next, args, anon_funcs);
            let (e2, next) = e2.transform(var2, next, args, anon_funcs);
            (e1, e2, next)
        };
        match self {
            Expr::Literal(l) => {
                args.type_consts.push((Type::Variable(var), l.get_type()));
                (iExpr::Literal(l), next)
            }
            Expr::Bound(s) => match args.scope.get(&s) {
                Some(ni) => {
                    let (path, t) = &*ni;
                    args.type_consts.push((Type::Variable(var), t.clone()));
                    (iExpr::Bound(path.clone()), next)
                }
                None => {
                    args.errors.push(Error::NameNotFound(s));
                    (iExpr::Error, next)
                }
            },
            Expr::BinOp(e1, op, e2) => {
                use self::BinOpcode::*;
                let (e1, e2, next) = match op {
                    Add | Sub | Mul | Div | Mod => {
                        args.type_consts.push((Type::Variable(var), Type::Int));
                        sequence(*e1, *e2, var, var, next, args, anon_funcs)
                    }
                    Greater | Less | GreaterEq | LessEq => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        args.type_consts.push((Type::Variable(next), Type::Int));
                        sequence(*e1, *e2, next, next, next + 1, args, anon_funcs)
                    }
                    Equal | NotEq => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, next, next, next + 1, args, anon_funcs)
                    }
                    And | Or => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, var, var, next, args, anon_funcs)
                    }
                };
                (iExpr::BinOp(Box::new(e1), op, Box::new(e2)), next)
            }
            Expr::UnOp(UnOpcode::Minus, e) => {
                args.type_consts.push((Type::Variable(var), Type::Int));
                let (e, next) = e.transform(var, next, args, anon_funcs);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            }
            Expr::UnOp(UnOpcode::Not, e) => {
                args.type_consts.push((Type::Variable(var), Type::Bool));
                let (e, next) = e.transform(var, next, args, anon_funcs);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            }
            Expr::Tuple(v) => {
                let mut v2 = Vec::new();
                let mut nnext = next + v.len() as u16;
                args.type_consts.push((
                    Type::Variable(var),
                    Type::Tuple(
                        (0..v.len())
                            .map(|i| Type::Variable(next + i as u16))
                            .collect(),
                    ),
                ));
                for (i, e) in v.into_iter().enumerate() {
                    let (e, next) = e.transform(next + i as u16, nnext, args, anon_funcs);
                    v2.push(e);
                    nnext = next;
                }
                (iExpr::Tuple(v2), nnext)
            }
            Expr::Application(e1, e2) => {
                args.type_consts.push((
                    Type::Variable(next),
                    Type::Function(
                        Box::new(Type::Variable(next + 1)),
                        Box::new(Type::Variable(var)),
                    ),
                ));
                let (e1, e2, next) = sequence(*e1, *e2, next, next, next + 1, args, anon_funcs);
                (iExpr::Application(Box::new(e1), Box::new(e2)), next)
            }
            Expr::Conditional(cond, e1, e2) => {
                args.type_consts.push((Type::Variable(next), Type::Bool));
                let (cond, next) = cond.transform(next, next + 1, args, anon_funcs);
                let (e1, e2, next) = sequence(*e1, *e2, var, var, next, args, anon_funcs);
                (
                    iExpr::Conditional(Box::new(cond), Box::new(e1), Box::new(e2)),
                    next,
                )
            }
            Expr::Function(v) => {
                let (closure, next) = fn_transform(v, next, next + 1, args, anon_funcs);
                anon_funcs.push(closure);
                (iExpr::Closure(anon_funcs.len() as u16 - 1), next)
            }
        }
    }
}

/// ### REQUIRES
/// count > 0
fn mk_curried_type(from: u16, count: u16) -> Type {
    let mut t = Type::Variable(from + count - 1);
    for i in (from..(count - 1)).rev() {
        t = Type::Function(Box::new(Type::Variable(i)), Box::new(t));
    }
    t
}

impl Literal {
    fn get_constraint(self) -> ConstraintValue {
        match self {
            Literal::Unit => panic!("trying to get constraint from unit"),
            Literal::Int(n) => ConstraintValue::Int(n),
            Literal::Bool(true) => ConstraintValue::Finite(0, 2),
            Literal::Bool(false) => ConstraintValue::Finite(1, 2),
            Literal::String(s) => ConstraintValue::Str(s),
        }
    }
}

impl Closure {
    fn substitute_types(&mut self, map: &HashMap<u16, Type>) {
        for (_, t) in &mut self.captures {
            t.substitute_type(&map);
        }
        for t in &mut self.args {
            t.substitute_type(&map);
        }
        self.return_type.substitute_type(&map);
    }
}

/// instantiate a type by substituting generics with variables
/// starting from var, Generic(n) => Variable(var + n)
/// # RETURNS
/// instantiated type and next free variable
fn gen2var(t: &Type, var: u16) -> (Type, u16) {
    match *t {
        Type::Unit | Type::Int | Type::Bool | Type::String => (t.clone(), var),
        Type::Constructor {
            ref arg,
            target,
            position,
        } => {
            let (arg, next) = gen2var(arg, var);
            (
                Type::Constructor {
                    arg: Box::new(arg),
                    target,
                    position,
                },
                next,
            )
        }
        Type::Function(ref from, ref to) => {
            let (from, next) = gen2var(from, var);
            let (to, nnext) = gen2var(to, var);
            (
                Type::Function(Box::new(from), Box::new(to)),
                max(next, nnext),
            )
        }
        Type::Generic(n) => (Type::Variable(var + n), var + n + 1),
        Type::Sum(n, ref t) => {
            let (t, next) = gen2var(&*t, var);
            (Type::Sum(n, Box::new(t)), next)
        }
        Type::Tuple(ref v) => {
            let (v, n): (Vec<Type>, Vec<u16>) = v.iter().map(|e| gen2var(e, var)).unzip();
            let n = n.into_iter().fold(var, |acc, elem| max(acc, elem));
            (Type::Tuple(v), n)
        }
        Type::Variable(_) => panic!("tried to instantiate instantiated type"),
    }
}

fn to_type(
    t: ProtoType,
    map: &HashMap<String, u16>, // map of type names -> index in types vector
    conver: &HashMap<String, u16>, // map of generics to variables
) -> Type {
    use self::ProtoType as P;
    use self::Type as T;
    match t {
        P::Unit => T::Unit,
        P::Int => T::Int,
        P::Bool => T::Bool,
        P::String => T::String,
        P::Tuple(v) => T::Tuple(v.into_iter().map(|t| to_type(t, map, conver)).collect()),
        P::Function(from, to) => T::Function(
            Box::new(to_type(*from, map, conver)),
            Box::new(to_type(*to, map, conver)),
        ),
        P::Generic(name) => match conver.get(&name) {
            Some(&n) => T::Generic(n),
            None => panic!("Error, generic not found"), // to be removed
        },
        P::Sum(name, t) => match map.get(&name) {
            Some(&n) => T::Sum(n, Box::new(to_type(*t, map, conver))),
            None => panic!("should be error type not defined"), // to be removed
        },
    }
}