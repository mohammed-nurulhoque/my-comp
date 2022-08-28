//! Author: Mohammed Nurul Hoque (2018)
//!
//! This module contains the logic for transforming a compilation unit from AST
//! to imperAST.

use std::collections::{BTreeMap, HashMap};

use crate::{
    ast::{Binding, Expr, Pattern},
    dtree::DTree,
    error::Error,
    imper_ast::{Closure, ConstraintValue, Expr as iExpr, Module, ValPath},
    namescope::NameScope,
    types::{BinOpcode, Literal, ProtoType, Type, UnOpcode,  TypeDecl},
    unify,
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mk_curried() {
        use self::Type::{Function, Variable};
        let t = mk_curried_type(5, 5);
        assert_eq!(
            t,
            Function(
                Box::new(Variable(5)),
                Box::new(Function(
                    Box::new(Variable(6)),
                    Box::new(Function(
                        Box::new(Variable(7)),
                        Box::new(Function(Box::new(Variable(8)), Box::new(Variable(9)),))
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_get_type_decl() {
        let vars = vec!["T"];
        let variants = vec![
            ("Nil", ProtoType::Unit),
            (
                "Node",
                ProtoType::Tuple(vec![
                    ProtoType::Sum(
                        "List",
                        Box::new(ProtoType::Generic("T")),
                    ),
                    ProtoType::Sum(
                        "List",
                        Box::new(ProtoType::Sum(
                            "BTree",
                            Box::new(ProtoType::Generic("T")),
                        )),
                    ),
                ]),
            ),
        ];
        let mut type_map = vec![("List", 0)].into_iter().collect();
        let mut ns = NameScope::new();
        let mut errors = Vec::new();
        let dec = get_type_decl("BTree", vars, variants, &mut type_map, &mut ns, &mut errors);
        assert_eq!(dec.name, "BTree");
        assert_eq!(dec.num_generics, 1);
        assert_eq!(
            dec.variants,
            vec![
                ("Nil", Type::Unit),
                (
                    "Node",
                    Type::Tuple(vec![
                        Type::Sum(0, vec!(Type::Generic(0))),
                        Type::Sum(0, vec!(Type::Sum(1, vec!(Type::Generic(0)))))
                    ])
                )
            ]
        );
        assert_eq!(type_map["BTree"], 1);
        assert_eq!(
            ns.get("Nil").unwrap(),
            &(
                ValPath::Constructor(1, 1),
                Type::Constructor {
                    target: 1,
                    position: 1,
                }
            )
        );
        assert_eq!(
            ns.get("Node").unwrap(),
            &((<>)
                ValPath::Constructor(1, 2),
                Type::Constructor {
                    target: 1,
                    position: 2,
                }
            )
        );
    }

    #[test]
    fn test_pattern() {
        use self::Pattern::*;
        let pat = Tuple(vec![
            SumVar(
                "cons",
                Box::new(Tuple(vec![Bind("x"), Bind("L1")])),
            ),
            SumVar(
                "cons",
                Box::new(Tuple(vec![Bind("y"), Bind("L2")])),
            ),
        ]);
        let mut ns = NameScope::new();
        ns.local().insert(
            "cons",
            (
                ValPath::Constructor(0, 2),
                Type::Constructor {
                    position: 2 as u16,
                    target: 0,
                },
            ),
        );
        let mut errors = Vec::new();
        let mut type_consts = Vec::new();
        let mut val_consts = BTreeMap::new();
        let mut type_decls = vec![TypeDecl {
            name: "List",
            num_generics: 1,
            variants: vec![
                ("nil", Type::Unit),
                (
                    "cons",
                    Type::Tuple(vec![Type::Generic(0), Type::Sum(0, vec![Type::Generic(0)])]),
                ),
            ],
        }];
        let mut closures = Vec::new();
        let mut path = vec![1];
        let mut args = Args {
            namescope: &mut ns,
            errors: &mut errors,
            type_consts: &mut type_consts,
            type_decls: &mut type_decls,
            closures: &mut closures,
        };
        pat.transform(
            10,
            20,
            &mut path,
            &mut args,
            ValPath::Local,
            &mut val_consts,
        );
        assert_eq!(
            args.namescope.get("x").unwrap(),
            &(ValPath::Local(vec![1, 0, 2, 0]), Type::Variable(24))
        );
        assert_eq!(
            args.namescope.get("L1").unwrap(),
            &(ValPath::Local(vec![1, 0, 2, 1]), Type::Variable(25))
        );
        assert_eq!(
            args.namescope.get("y").unwrap(),
            &(ValPath::Local(vec![1, 1, 2, 0]), Type::Variable(28))
        );
        assert_eq!(
            args.namescope.get("L2").unwrap(),
            &(ValPath::Local(vec![1, 1, 2, 1]), Type::Variable(29))
        );
        assert!(args.errors.is_empty());
        assert_eq!(path, &[1]);
    }
}


/// A pair of types. Just more expressive
type TypeConstraint = (Type, Type);

pub struct TypingContext<'input> {
    type_decls: Vec<TypeDecl<'input>>,
    closures: Vec<Closure<'input>>,
    globals: Vec<(iExpr<'input>, BTreeMap<ValPath, ConstraintValue<'input>>, Type)>,
    namescope: NameScope<'input>,
    type_consts: Vec<TypeConstraint>,
    type_map: HashMap<&'input str, u16>,
    errors: Vec<Error<'input>>,
    next: usize,
}

impl<'input> TypingContext<'input> {

    pub fn new() -> Self {
        TypingContext {
            type_decls: vec![],
            closures: vec![],
            globals: vec![], 
            namescope: NameScope::new(),
            type_consts: vec![], 
            type_map: HashMap::new(), 
            errors: vec![],
            next: 0,
        }
    }

    pub fn export(mut self) -> Result<Module<'input>, Error<'input>> {
        Ok(Module {
            closures: self.closures,
            globals: self.globals,
            type_decls: self.type_decls,
            globals_names: self.namescope
                .pop_layer()
                .into_iter()
                .map(|(s, (path, _))| (s, path))
                .collect(),
        })
    }

    pub fn add_imports(&mut self, map: HashMap<&'input str, (ValPath, Type)>) {
        self.namescope.extend_local(map);
    }

    pub fn add_constr(&mut self, var: u16, constraint: Type) {
        self.type_consts.push((Type::Variable(var), constraint));
    }

    pub fn add_binding(&mut self, binding: Binding<'input>) -> Result<(), Error<'input>> {
        match binding {
            Binding::Type { name, vars, variants } => {
                let t = self.get_type_decl(name, vars, variants);
                self.type_decls.push(t);
            }
            Binding::Value(pat, expr, is_rec) => {
                let tuple = self.binding_transform(self.globals.len() as u16, pat, expr, is_rec)?;
                self.globals.push(tuple)
            },
            Binding::Method(type_name, method_name, body) => panic!("Method Definition"),
        }
        Ok(())
    }

    /// Transform a top-level binding
    /// # Arguments
    /// - order in all top-level value bindings (the valpath)
    /// - pattern
    /// - expression
    /// - is_rec: is recursive? if recursive, pattern added to scope before the expression
    /// 
    /// # Returns
    /// Result(tranformed expression, constraints on the expression by the pattern, type of expression)
    /// 
    /// # Future
    /// when non-top-level bindings are allowed, shouldn't generalize types here

    fn binding_transform(
        &mut self,
        order: u16,
        pat: Pattern<'input>,
        expr: Expr<'input>,
        is_rec: bool,
    ) -> Result<(iExpr<'input>, BTreeMap<ValPath, ConstraintValue<'input>>, Type), Error<'input>> {
        let mut path = vec![order];
        let mut val_consts = BTreeMap::new();
        // remember how many closures was already there. Closures are added to global closures vector
        // as the expression is processed, i.e. before type unification. This means we have to change
        // their types inside the global vector
        let closures_num = self.closures.len();
        // we don't insert directly into the scope because we want to do type unification
        // before inserting finally
        let expr = if is_rec {
            self.namescope.push_layer();
            let next = pat.transform(0, 1, &mut path, self, ValPath::StaticVal, &mut val_consts);
            expr.transform(0, next, self).0
        } else {
            let (e, next) = expr.transform(0, 1, self);
            self.namescope.push_layer();
            pat.transform(0, next, &mut path, self, ValPath::StaticVal, &mut val_consts);
            e
        };
        let mut type_consts = self.type_consts.drain(0..).collect();
        let mut map = unify::unify(&mut type_consts)?;
        let mut local = self.namescope.pop_layer();
        for (_, (_, t)) in local.iter_mut() {
            t.substitute_vars(&map);
            t.generalize_type();
        }
        self.namescope.extend_local(local);

        // chnage types of closures added for this binding
        for closure in self.closures.iter_mut().skip(closures_num) {
            closure.substitute_types(&map);
        }

        let mut t = Type::Variable(0);
        t.substitute_vars(&mut map);
        t.generalize_type();
        // let mut pretty = String::new();
        // t.pretty_format(&mut pretty, args.type_decls);
        // println!("{}",pretty);
        Ok((expr, val_consts, t))
    }

    
    fn get_type_decl(&mut self,
        name: &'input str,
        vars: Vec<&'input str>,
        variants: Vec<(&'input str, ProtoType<'input>)>,
    ) -> TypeDecl<'input> {
        let generics_map: HashMap<&'input str, u16> = vars
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i as u16))
            .collect();
        let len = self.type_map.len() as u16;
        self.type_map.insert(name, len);
        TypeDecl {
            name,
            num_generics: generics_map.len() as u16,
            variants: variants
                .into_iter()
                .enumerate()
                .map(|(i, (s, t))| {
                    let t = match t.to_type(&self.type_map, &generics_map) {
                        Ok(t) => t,
                        Err(e) => { self.errors.push(e); Type::Unit }
                    };
                    self.namescope.local().insert(
                        s,
                        (
                            ValPath::Constructor(len, (i+1) as u16),
                            Type::Constructor {
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

    fn fn_transform(&mut self,
        fn_branches: Vec<(Vec<Pattern<'input>>, Expr<'input>)>,
        var: u16,
        next: u16,
    ) -> (u16, u16) {
        // patterns per branch
        let len = fn_branches[0].0.len() as u16;
        debug_assert!(len > 0);
        self.type_consts
            .push((Type::Variable(var), mk_curried_type(next, len + 1)));
        let mut nnext = next + len + 1;
        let mut dtree = DTree::new();
        let mut branches = Vec::new();
        self.namescope.push_layer();
        for (i, (pats, e)) in fn_branches.into_iter().enumerate().rev() {
            if pats.len() as u16 != len {
                self.errors.push(Error::VariablePatsNum);
            }
    
            let mut path = vec![];
            let mut val_consts = BTreeMap::new();
            for (j, pat) in pats.into_iter().enumerate() {
                path.push(j as u16);
                nnext = pat.transform(
                    next + j as u16,
                    nnext,
                    &mut path,
                    self,
                    ValPath::Local,
                    &mut val_consts,
                );
                path.pop();
            }
            dtree.add_pattern(val_consts, i as u16);
            let (e, tmp) = e.transform(next + len, nnext, self);
            branches.push(e);
            nnext = tmp;
            self.namescope.drain_local();
        }
        let map = self.namescope.pop_layer();
        let mut captures = Vec::new();
        for (_, (val, t)) in map.into_iter() {
            match val {
                ValPath::CaptureCaptured(n, _) | ValPath::CaptureLocal(n, _) => {
                    captures.push((n, (val, t)))
                }
                _ => panic!("non capture value path not expected here"),
            }
        }
        captures.sort_unstable_by(|(ord1, _), (ord2, _)| ord1.cmp(ord2));
        let captures: Vec<(ValPath, Type)> = captures.into_iter().map(|(_, v)| v).collect();
        let is_static = captures.is_empty();
        self.closures.push(Closure {
            captures,
            dtree,
            branches: branches.into_iter().rev().collect(),
            args: (next..(next + len)).map(|n| Type::Variable(n)).collect(),
            return_type: Type::Variable(next + len),
        });
        if is_static {}
    
        ((self.closures.len() - 1) as u16, nnext)
    }
}

impl<'input> Pattern<'input> {
    /// parse a pattern and fill local with the name bindings, and val_consts with
    /// value bindings.
    /// ### RETURNS
    /// next free variable
    fn transform<'b, T: Fn(Vec<u16>) -> ValPath + Copy>(
        self,
        var: u16,
        next: u16,
        path: &mut Vec<u16>,
        ctx: &mut TypingContext<'input>,
        valpath_constructor: T,
        val_consts: &mut BTreeMap<ValPath, ConstraintValue<'input>>,
    ) -> u16 {
        match self {
            Pattern::Error(..) => panic!("Parse Error not supposed to be propagated"),
            Pattern::Wild => next,
            Pattern::Literal(l) => {
                ctx.add_constr(var, l.get_type());
                if let Literal::Unit = l {
                    ()
                } else {
                    val_consts.insert(valpath_constructor(path.clone()), l.get_constraint());
                }
                next
            }
            Pattern::Bind(s) => match ctx.namescope.local().get(&s) {
                Some(_) => {
                    ctx.errors.push(Error::MultBindPattern(s));
                    next
                }
                None => {
                    ctx.namescope
                        .local()
                        .insert(s, (valpath_constructor(path.clone()), Type::Variable(var)));
                    next
                }
            },
            Pattern::Tuple(v) => {
                let len = v.len() as u16;
                let mut nnext = next + len;
                ctx.add_constr(var, Type::Tuple((next..nnext).map(|i| Type::Variable(i)).collect()));
                for (i, pat) in v.into_iter().enumerate() {
                    let i = i as u16;
                    path.push(i);
                    nnext =
                        pat.transform(next + i, nnext, path, ctx, valpath_constructor, val_consts);
                    path.pop();
                }
                nnext
            }
            Pattern::SumVar(constructor, pat) => match ctx.namescope.get(&constructor) {
                None => {
                    ctx.errors.push(Error::ConstructorNotFound(constructor));
                    next
                }
                Some(ni) => {
                    if let Type::Constructor { target, position } = ni.1 {
                        let t = &ctx.type_decls[target as usize];
                        // The value constraint for the tag
                        val_consts.insert(
                            valpath_constructor({
                                let mut p = path.clone();
                                p.push(0);
                                p
                            }),
                            // position starts from 1
                            ConstraintValue::Finite(position - 1, t.variants.len() as u16),
                        );

                        let (from, n1) = t.variants[position as usize - 1].1.instantiate(next + 1);
                        let (to, n2) = (
                            Type::Sum(
                                target,
                                (0..t.num_generics)
                                    .map(|n| Type::Variable(next + 1 + n))
                                    .collect(),
                            ),
                            next + 1 + t.num_generics,
                        );
                        ctx.add_constr(var, to);
                        ctx.add_constr(next, from);
                        path.push(position);
                        debug_assert!(n2 >= n1);
                        let next =
                            pat.transform(next, n2, path, ctx, valpath_constructor, val_consts);
                        path.pop();
                        next
                    } else {
                        ctx.errors.push(Error::NonConstAppPattern(constructor));
                        next
                    }
                }
            },
        }
    }
}

impl<'input> Expr<'input> {
    fn transform(self, var: u16, next: u16, ctx: &mut TypingContext<'input>) -> (iExpr<'input>, u16) {
        let sequence = |e1: Expr<'input>, e2: Expr<'input>, var1, var2, next, ctx: &mut TypingContext<'input>| {
            let (e1, next) = e1.transform(var1, next, ctx);
            let (e2, next) = e2.transform(var2, next, ctx);
            (e1, e2, next)
        };
        match self {
            Expr::Error(..) => panic!("Parse Error not supposed to be propagated"),
            Expr::Literal(l) => {
                ctx.add_constr(var, l.get_type());
                (iExpr::Literal(l), next)
            }
            Expr::Bound(s) => match ctx.namescope.get(&s) {
                Some(ni) => {
                    let (path, t) = &*ni;
                    let (t, next) = if let Type::Constructor { target, position } = t {
                        let ttype = &ctx.type_decls[*target as usize];
                        let (from, n1) = ttype.variants[*position as usize - 1].1.instantiate(next);
                        let (to, n2) = (
                            Type::Sum(
                                *target,
                                (0..ttype.num_generics)
                                    .map(|n| Type::Variable(next + n))
                                    .collect(),
                            ),
                            next + ttype.num_generics,
                        );
                        debug_assert!(n2 >= n1);
                        (Type::Function(Box::new(from), Box::new(to)), n2)
                    } else {
                        t.instantiate(next)
                    };
                    // borrow checker doesn't accept ctx.add_constr here
                    ctx.type_consts.push((Type::Variable(var), t));
                    (iExpr::Bound(path.clone()), next)
                }
                None => {
                    ctx.errors.push(Error::NameNotFound(s));
                    (iExpr::Error, next)
                }
            },
            Expr::Slice(e1, e2, e3) => {
                ctx.add_constr(var, Type::String);
                ctx.add_constr(next, Type::Int);
                let (e1, nnext) = e1.transform(var, next+1, ctx);
                let (e2, nnext) = e2.transform(next, nnext, ctx);
                let (e3, nnext) = e3.transform(next, nnext, ctx);
                (iExpr::Slice(Box::new(e1), Box::new(e2), Box::new(e3)), nnext)
            },
            Expr::BinOp(e1, op, e2) => {
                use self::BinOpcode::*;
                let (e1, e2, next) = match op {
                    Index => {
                        ctx.add_constr(var, Type::Int);
                        ctx.add_constr(next, Type::String);
                        sequence(*e1, *e2, next, var, next+1, ctx)
                    }
                    Add | Sub | Mul | Div | Mod => {
                        ctx.add_constr(var, Type::Int);
                        sequence(*e1, *e2, var, var, next, ctx)
                    }
                    Greater | Less | GreaterEq | LessEq => {
                        ctx.add_constr(var, Type::Bool);
                        ctx.add_constr(next, Type::Int);
                        sequence(*e1, *e2, next, next, next + 1, ctx)
                    }
                    Concat => {
                        ctx.add_constr(var, Type::String);
                        sequence(*e1, *e2, var, var, next, ctx)
                    }
                    Equal | NotEq => {
                        ctx.add_constr(var, Type::Bool);
                        sequence(*e1, *e2, next, next, next + 1, ctx)
                    }
                    And | Or => {
                        ctx.add_constr(var, Type::Bool);
                        sequence(*e1, *e2, var, var, next, ctx)
                    }
                };
                (iExpr::BinOp(Box::new(e1), op, Box::new(e2)), next)
            }
            Expr::UnOp(UnOpcode::Minus, e) => {
                ctx.add_constr(var, Type::Int);
                let (e, next) = e.transform(var, next, ctx);
                (iExpr::UnOp(UnOpcode::Minus, Box::new(e)), next)
            }
            Expr::UnOp(UnOpcode::Not, e) => {
                ctx.add_constr(var, Type::Bool);
                let (e, next) = e.transform(var, next, ctx);
                (iExpr::UnOp(UnOpcode::Not, Box::new(e)), next)
            }
            Expr::Tuple(v) => {
                let mut nnext = next + v.len() as u16;
                ctx.type_consts.push((
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
                    let (e, next) = e.transform(next + i as u16, nnext, ctx);
                    v2.push(e);
                    nnext = next;
                }
                (iExpr::Tuple(v2), nnext)
            }
            Expr::Application(e1, e2) => {
                // TODO : if e1 is constructor ...
                if let Expr::Bound(s) = *e1 {
                    match ctx.namescope.get(s) {
                        Some((_, Type::Constructor { .. })) => (), // unimplemented!(),
                        Some(_) => (), // unimplemented!(),
                        None => (), // unimplemented!(),
                    }
                }
                ctx.type_consts.push((
                    Type::Variable(next),
                    Type::Function(
                        Box::new(Type::Variable(next + 1)),
                        Box::new(Type::Variable(var)),
                    ),
                ));
                let (e1, e2, next) = sequence(*e1, *e2, next, next + 1, next + 2, ctx);
                (iExpr::Application(Box::new(e1), Box::new(e2)), next)
            }
            Expr::MethodCall(object, method) => panic!("Method Call"),
            Expr::Conditional(cond, e1, e2) => {
                ctx.add_constr(next, Type::Bool);
                let (cond, next) = cond.transform(next, next + 1, ctx);
                let (e1, e2, next) = sequence(*e1, *e2, var, var, next, ctx);
                (
                    iExpr::Conditional(Box::new(cond), Box::new(e1), Box::new(e2)),
                    next,
                )
            }
            Expr::Closure(v) => {
                let (idx, next) = ctx.fn_transform(v, var, next);
                (iExpr::Closure(idx), next)
            }
        }
    }
}

/// ### REQUIRES
/// count > 0
fn mk_curried_type(from: u16, count: u16) -> Type {
    let mut t = Type::Variable(from + count - 1);
    for i in (from..(from + count - 1)).rev() {
        t = Type::Function(Box::new(Type::Variable(i)), Box::new(t));
    }
    t
}

impl<'input> Literal<'input> {
    fn get_constraint(self) -> ConstraintValue<'input> {
        match self {
            Literal::Unit => panic!("trying to get constraint from unit"),
            Literal::Int(n) => ConstraintValue::Int(n),
            Literal::Bool(true) => ConstraintValue::Finite(0, 2),
            Literal::Bool(false) => ConstraintValue::Finite(1, 2),
            Literal::String(s) => ConstraintValue::Str(s),
        }
    }
}

impl<'input> Closure<'input> {
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
