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
    types::{Type, Literal, BinOpcode, UnOpcode},
};

/// represents a compilation module (a single file)
pub struct Module {
    /// all closures including top-level functions
    pub closures: Vec<(Closure)>,
    /// vector of global values, each declaration is a single value
    /// e.g. (x, y) = (1, 2) is a single value. The BTreeMap has any
    /// literal constraints on global values e.g.
    /// (1, 2) = f 5;
    pub globals: Vec<(Expr, BTreeMap<ValPath, ConstraintValue>, Type)>,
    /// path of each global name including top-level functions
    pub globals_names: HashMap<String, ValPath>,
    pub type_decls: Vec<TypeDecl>,
}

/// The path of a value. Together with the type, it can give the actual position
/// of the value in memory. The type is union of the storage classes of values.
/// The vec<u16> identifies the position of the value in a compound value.
/// In a tuple (x0, x1, ..., x_n), if the path of tuple is p, the path of xi is [..p, i]
/// In a sum(i, v) that has path p, [..p, 0] is the path of the tag of the type and
/// [..p, i] is the path for the v in the ith variant. This means, [..p, 0] should be checked
/// before accessing [..p, i] otherwise it can be unsafe.
/// A captured values has index in captured values and the capture path in parent scope
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
    /// - path in parent scope
    CaptureLocal(u16, Vec<u16>),
    /// a global value, 1st arg is index in Module.globals and 2nd is path in its expr
    StaticVal(Vec<u16>),
    /// just a marker, constructors are not stored anywhere
    Constructor,
}

/// Representation of a sum type
pub struct TypeDecl {
    pub name: String,
    /// number of types on which this is generic
    pub num_generics: u16,
    pub variants: Vec<(String,Type)>,
}

/// Represents both static (top-level functions) and dynamic closures
pub struct Closure {
    /// values captured from parent
    pub captures: Vec<(ValPath, Type)>,
    pub args: Vec<Type>,
    pub return_type: Type,
    /// decision tree of args pattern matching
    pub dtree: DTree,
    pub branches: Vec<Expr>,
}

/// A pattern is a set of constraints on a value, which are categorized as follows
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ConstraintValue {
    /// nth option out of x finitely many option, includes Booleans and union tags
    /// indexing starting from 1 upto x.
    Finite(u16, u16),
    /// integer constraint which is technically finite but represented sparsely, so
    /// is practically inifinite
    Int(isize),
    /// string constraint, we allow strings in pattern matching
    Str(String),
}

/// Representation of an expression
pub enum Expr {
    Literal(Literal),
    /// named value
    Bound(ValPath),
    Tuple(Vec<Expr>),

    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UnOp(UnOpcode, Box<Expr>),

    /// closure which is an index into Module.anon_funcs
    Closure(u16),
    /// Apply e1 on e2
    Application(Box<Expr>, Box<Expr>),

    /// if e1 then e2 else e3
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),

    Error,
}