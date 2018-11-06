//! Author: Mohammed Nurul Hoque (2018)
//! 
//! This module contains the logic for transforming a compilation unit from AST
//! to imperAST.

use std::collections::{ HashMap, BTreeMap };
use crate::{
    ast::{Binding, Expr, Pattern},
    imper_ast::{Closure, ConstraintValue, Expr as iExpr, Module, TypeDecl, ValPath},
    dtree::DTree,
    types::{BinOpcode, Literal, ProtoType, Type, UnOpcode},
    unify,
    namescope::NameScope,
    error::Error,
};

#[cfg(test)]
mod test {
    #[test]

    fn test_mk_curried() {
        use super::Type::{ Function, Variable };
        let t = super::mk_curried_type(5, 5);
        assert_eq!(t, Function(
            Box::new(Variable(5)), 
            Box::new(Function(
                Box::new(Variable(6)),
                Box::new(Function(
                    Box::new(Variable(7)),
                    Box::new(Function(
                        Box::new(Variable(8)),
                        Box::new(Variable(9)),
                    ))
                ))
            ))
        ));
    }
}
/// The transformation function, takes a series of bindings in AST form,
/// which are either value binding or type declarations. Converts to
/// a Module struct (see imper_ast.rs) which separated functions and
/// variables.
pub fn ast2imper_ast(bindings: Vec<Binding>) -> Result<Module, Error> {
    let mut errors = Vec::new();
    let mut type_map = HashMap::new();
    let mut closures = Vec::new();
    let mut global_scope = NameScope::new();
    let mut globals = Vec::new();
    let mut type_decls = Vec::new();
    let mut type_consts = Vec::new();
    let mut val_order = 0;

    for binding in bindings {
        let mut args = Args {
            type_decls: &mut type_decls,
            closures: &mut closures,
            namescope: &mut global_scope,
            type_consts: &mut type_consts,
            errors: &mut errors,
        };
        match binding {
            Binding::Type { name, vars, variants,} => args.type_decls.push(
                get_type_decl(name, vars, variants, &mut type_map, args.namescope,
            )),
            Binding::Value(pat, expr, is_rec) => {
                globals.push(binding_transform(
                    val_order,
                    pat,
                    expr,
                    is_rec,
                    &mut args,
                )?);
                val_order += 1;
            }
        }
    }

    Ok(Module {
        closures, globals, type_decls,
        globals_names: global_scope.pop_layer().into_iter()
            .map(|(s, (path, _))| (s, path)).collect(),
    })
}

/// just more expressive
type TypeConstraint = (Type, Type);

/// wraps arguments for conciseness
struct Args<'a> {
    type_decls: &'a mut Vec<TypeDecl>,
    closures: &'a mut Vec<Closure>,
    namescope: &'a mut NameScope,
    type_consts: &'a mut Vec<TypeConstraint>,
    errors: &'a mut Vec<Error>,
}

/// This code is a real mess, needs refactoring
fn binding_transform<'a, 'b>(
    order: u16,
    pat: Pattern,
    expr: Expr,
    is_rec: bool,
    args: &mut Args<'a>,
) -> Result<(iExpr, BTreeMap<ValPath, ConstraintValue>, Type), Error> {
    let mut path = vec![order];
    let mut val_consts = BTreeMap::new();
    // we don't insert directly into the scope because we want to do type inference
    // before inserting finally
    let expr = if is_rec {
        args.namescope.push_layer();
        let next = pat.transform(
            0,
            1,
            &mut path,
            args,
            ValPath::StaticVal,
            &mut val_consts,
        );
        expr.transform(0, next, args).0
    } else {
        let next = pat.transform(
            0,
            1,
            &mut path,
            args,
            ValPath::StaticVal,
            &mut val_consts,
        );
        args.namescope.push_layer();
        expr.transform(0, next, args).0
    };
    let mut type_consts = args.type_consts.drain(0..).collect();
    let mut map = unify::unify(&mut type_consts)?;
    let local = args.namescope.pop_layer();
    args.namescope.extend_local(local);

    let mut t = Type::Variable(0);
    t.substitute_vars(&mut map);
    Ok((expr, val_consts, t))
}

fn get_type_decl(
    name: String,
    vars: Vec<String>,
    variants: Vec<(String, ProtoType)>,
    type_map: &mut HashMap<String, u16>,
    namescope: &mut NameScope,
) -> TypeDecl {
    let generics_map: HashMap<String, u16> = vars
        .into_iter()
        .enumerate()
        .map(|(i, s)| (s, i as u16))
        .collect();
    let len = type_map.len() as u16;
    type_map.insert(name.clone(), len);
    TypeDecl {
        name,
        num_generics: generics_map.len() as u16,
        variants: variants
            .into_iter()
            .enumerate()
            .map(|(i, (s, t))| {
                let t = t.to_type(type_map, &generics_map);
                namescope.local().insert(
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

fn fn_transform<'a, 'b> (
    fn_branches: Vec<(Vec<Pattern>, Expr)>,
    var: u16,
    next: u16,
    args: &mut Args<'a>,
) -> (u16, u16) {
    // patterns per branch
    let len = fn_branches[0].0.len() as u16;
    debug_assert!(len > 0);
    args.type_consts
        .push((Type::Variable(var), mk_curried_type(next, len + 1)));
    let mut nnext = next + len + 1;
    let mut dtree = DTree::new();
    let mut branches = Vec::new();
    args.namescope.push_layer();
    for (i, (pats, e)) in fn_branches.into_iter().enumerate().rev() {
        if pats.len() as u16 != len {
            args.errors.push(Error::VariablePatsNum);
        }

        let mut path = vec![];
        let mut val_consts = BTreeMap::new();
        for (j, pat) in pats.into_iter().enumerate() {
            path.push(j as u16);
            pat.transform(
                next + j as u16,
                nnext,
                &mut path,
                args,
                ValPath::Local,
                &mut val_consts,
            );
            path.pop();
        }
        dtree.add_pattern(val_consts, i as u16);
        let (e, tmp) = e.transform(next + len, nnext, args);
        branches.push(e);
        nnext = tmp;
        args.namescope.drain_local();
    }
    let map = args.namescope.pop_layer();
    let mut captures = Vec::new();
    for (_, (val, t)) in map.into_iter() {
        match val {
            ValPath::CaptureCaptured(n, _) | ValPath::CaptureLocal(n, _) => captures.push((n, (val, t))),
            _ => panic!("non capture not expected here"),
        }
    }
    captures.sort_unstable_by(|(ord1, _), (ord2, _)| ord1.cmp(ord2));
    let captures: Vec<(ValPath, Type)> = captures.into_iter().map(|(_, v)| v).collect();
    let is_static = captures.is_empty();
    args.closures.push(Closure {
        captures,
        dtree,
        branches: branches.into_iter().rev().collect(),
        args: (next..(next + len)).map(|n| Type::Variable(n)).collect(),
        return_type: Type::Variable(next + len),
    });
    if is_static {

    }

    ((args.closures.len() - 1) as u16, nnext)
}

impl Pattern {
    /// parse a pattern and fill local with the name bindings, and val_consts with
    /// value bindings.
    /// ### RETURNS
    /// next free variable
    fn transform<'a, 'b, T: Fn(Vec<u16>) -> ValPath + Copy>(
        self,
        var: u16,   
        next: u16,
        path: &mut Vec<u16>,
        args: &mut Args<'a>,
        valpath_constructor: T,
        val_consts: &mut BTreeMap<ValPath, ConstraintValue>,
    ) -> u16 {
        match self {
            Pattern::Wild => next,
            Pattern::Literal(l) => {
                args.type_consts.push((Type::Variable(var), l.get_type()));
                if let Literal::Unit = l {
                    ()
                } else {
                    val_consts.insert(valpath_constructor(path.clone()), l.get_constraint());
                }
                next
            }
            Pattern::Bind(s) => match args.namescope.local().get(&s) {
                Some(_) => {
                    args.errors.push(Error::MultBindPattern(s));
                    next },
                None => {
                    args.namescope.local().insert(s, (valpath_constructor(path.clone()), Type::Variable(var)));
                    next },
            },
            Pattern::Tuple(v) => {
                let len = v.len() as u16;
                let mut nnext = next + len;
                args.type_consts.push((
                    Type::Variable(var),
                    Type::Tuple((next..nnext).map(|i| Type::Variable(i)).collect()),
                ));
                for (i, pat) in v.into_iter().enumerate() { let i = i as u16; path.push(i); nnext = pat.transform(next + i, nnext, path, args, valpath_constructor, val_consts); path.pop();
                }
                nnext
            }
            Pattern::SumVar(constructor, pat) => match args.namescope.get(&constructor) {
                None => {
                    args.errors.push(Error::ConstructorNotFound(constructor));
                    next
                }
                Some(ni) => if let Type::Constructor { ref arg, target, position } = ni.1 {
                    let t = &args.type_decls[target as usize];
                    // The value constraint for the tag
                    val_consts.insert(
                        valpath_constructor({let mut p = path.clone(); p.push(0); p}),
                        // position starts from 1 but need 0 indexing
                        ConstraintValue::Finite(position, t.variants.len() as u16),
                    );

                    let (from, n1) = arg.instantiate(next + 1);
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
                    debug_assert!(n2 > n1);
                    let next = pat.transform(
                        next,
                        n2,
                        path,
                        args,
                        valpath_constructor,
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
    fn transform<'a, 'b>(
        self,
        var: u16,
        next: u16,
        args: &mut Args<'a>,
    ) -> (iExpr, u16) {
        let sequence = |e1: Expr,
                        e2: Expr,
                        var1,
                        var2,
                        next,
                        args: &mut Args<'a>| {
            let (e1, next) = e1.transform(var1, next, args);
            let (e2, next) = e2.transform(var2, next, args);
            (e1, e2, next)
        };
        match self {
            Expr::Literal(l) => {
                args.type_consts.push((Type::Variable(var), l.get_type()));
                (iExpr::Literal(l), next)
            }
            Expr::Bound(s) => match args.namescope.get(&s) {
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
                        sequence(*e1, *e2, var, var, next, args)
                    }
                    Greater | Less | GreaterEq | LessEq => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        args.type_consts.push((Type::Variable(next), Type::Int));
                        sequence(*e1, *e2, next, next, next + 1, args)
                    }
                    Equal | NotEq => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, next, next, next + 1, args)
                    }
                    And | Or => {
                        args.type_consts.push((Type::Variable(var), Type::Bool));
                        sequence(*e1, *e2, var, var, next, args)
                    }
                };
                (iExpr::BinOp(Box::new(e1), op, Box::new(e2)), next)
            }
            Expr::UnOp(UnOpcode::Minus, e) => {
                args.type_consts.push((Type::Variable(var), Type::Int));
                let (e, next) = e.transform(var, next, args);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            }
            Expr::UnOp(UnOpcode::Not, e) => {
                args.type_consts.push((Type::Variable(var), Type::Bool));
                let (e, next) = e.transform(var, next, args);
                (iExpr::UnOp(UnOpcode::Not, Box::new(e)), next)
            }
            Expr::Tuple(v) => {
                let mut nnext = next + v.len() as u16;
                args.type_consts.push((
                    Type::Variable(var),
                    Type::Tuple(
                        (0..v.len())
                            .map(|i| Type::Variable(next + i as u16))
                            .collect(),
                    ),
                ));
                let mut v2 = Vec::new();
                for (i, e) in v.into_iter().enumerate() {
                    // the rhs next is not the outer next, otherwise cannot update mutable nnext
                    let (e, next) = e.transform(next + i as u16, nnext, args);
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
                let (e1, e2, next) = sequence(*e1, *e2, next, next + 1, next + 2, args);
                (iExpr::Application(Box::new(e1), Box::new(e2)), next)
            }
            Expr::Conditional(cond, e1, e2) => {
                args.type_consts.push((Type::Variable(next), Type::Bool));
                let (cond, next) = cond.transform(next, next + 1, args);
                let (e1, e2, next) = sequence(*e1, *e2, var, var, next, args);
                (
                    iExpr::Conditional(Box::new(cond), Box::new(e1), Box::new(e2)),
                    next,
                )
            }
            Expr::Closure(v) => {
                let (idx, next) = fn_transform(v, next, next + 1, args);
                (iExpr::Closure(idx), next)
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
            t.substitute_vars(&map);
        }
        for t in &mut self.args {
            t.substitute_vars(&map);
        }
        self.return_type.substitute_vars(&map);
    }
}
