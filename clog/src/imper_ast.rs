//! This module defines a typed AST, which is the second intermediate form,
//! It's genereated from the AST created by the parser.
//! 
//! 

use std::collections::{
    HashMap,
    BTreeMap,
};
use crate::{
    dtree::DTree,
    types::{Type, Literal, BinOpcode, UnOpcode, TypeDecl},
};

/// represents a compilation module (a single file)
#[derive(Debug)]
pub struct Module<'input> {
    /// all closures including top-level functions
    pub closures: Vec<Closure<'input>>,
    /// vector of global values, each declaration is a single value
    /// e.g. (x, y) = (1, 2) is a single value. The BTreeMap has any
    /// literal constraints on global values e.g.
    /// (1, 2) = f 5;
    pub globals: Vec<(Expr<'input>, BTreeMap<ValPath, ConstraintValue<'input>>, Type)>,
    /// path of exported global name including top-level functions
    pub globals_names: HashMap<&'input str, ValPath>,
    pub type_decls: Vec<TypeDecl<'input>>,
}

/// The path of a value. Together with the type, it can give the actual position
/// of the value in memory. The type is union of the storage classes of values.
/// The vec<u16> identifies the position of the value in a compound value.
/// In a tuple (x0, x1, ..., x_n), if the path of tuple is p, the path of xi is [..p, i]
/// In a sum(i, v) that has path p, [..p, 0] is the path of the tag of the type and
/// [..p, i] is the path for the v in the ith variant. This means, [..p, 0] should be checked
/// before accessing [..p, i] otherwise it can be unsafe.
/// A captured value has index in captured values and the capture path in parent scope
/// if a closure captures a value from a higher scope, all closures in between have to capture it.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ValPath {
    /// value in local stack
    Local(Vec<u16>),
    /// capture a value that is captured in parent scope
    /// ### Args
    /// - index in captured values
    /// - index in parent scope
    CaptureCaptured(u16, u16),
    /// capture a value that is local to the parent scope
    /// ### Args
    /// - index in captured values
    /// - path in parent scope unwrapped from Local()
    CaptureLocal(u16, Vec<u16>),
    /// a global value, 
    StaticVal(Vec<u16>),
    /// just a marker, constructors are not stored anywhere
    Constructor(u16, u16),
}

/// Represents both static (top-level functions) and dynamic closures
#[derive(Debug)]
pub struct Closure<'input> {
    /// values captured from parent
    pub captures: Vec<(ValPath, Type)>,
    pub args: Vec<Type>,
    pub return_type: Type,
    /// decision tree of args pattern matching
    pub dtree: DTree<'input>,
    pub branches: Vec<Expr<'input>>,
}

/// A pattern is a set of constraints on a value, which are categorized as follows
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ConstraintValue<'input> {
    /// nth option out of x finitely many option, includes Booleans and union tags
    /// indexing starting from 1 upto x.
    Finite(u16, u16),
    /// integer constraint which is technically finite but represented sparsely, so
    /// is practically inifinite
    Int(isize),
    /// string constraint, we allow strings in pattern matching
    Str(&'input str),
}

/// Representation of an expression
#[derive(Debug)]
pub enum Expr<'input> {
    Literal(Literal<'input>),
    /// named value
    Bound(ValPath),
    Tuple(Vec<Expr<'input>>),

    BinOp(Box<Expr<'input>>, BinOpcode, Box<Expr<'input>>),
    UnOp(UnOpcode, Box<Expr<'input>>),

    /// closure which is an index into Module.anon_funcs
    Closure(u16),
    /// Apply e1 on e2
    Application(Box<Expr<'input>>, Box<Expr<'input>>),
    /// Constructor Application
    SumVal {
        target: u16,
        position: u16,
        value: Box<Expr<'input>>,
    },

    /// if e1 then e2 else e3
    Conditional(Box<Expr<'input>>, Box<Expr<'input>>, Box<Expr<'input>>),

    Error,
}