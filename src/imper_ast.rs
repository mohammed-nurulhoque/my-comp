//! This module defines a typed AST, which is the second intermediate form,
//! It's genereated from the AST created by the pareser.
//! 
//! item path convention:
//! 0.xyz: path xyz in function arguments
//! 1.xyz: path xyz in captured arguments
//! 2.xyz: path xyz in static values in current modules
//! n.xyz >= 3.xyz: path xyz in nth imported module (future)
//! path.x where path is tuple: xth element in tuple
//! path.0 where path is a sum variant: discriminant of sum
//! path.n >= path.1 where path is sum: argument of nth variant of sum
//! 
//! The module represents a compilation unit as a 2-tuple:
//! v1: HashSet<(String,Type,usize)>: Constructor name from type to ith type
//! v2: Vec<(String, Expr, Type)>: exported values

use std::collections::{
    HashMap,
    BTreeMap,
};
use dtree::DTree;
use types::{Type, Literal, BinOpcode, UnOpcode};

pub struct Module {
    pub static_funcs: Vec<(String, Closure)>,
    pub anon_funcs: Vec<Closure>,
    pub globals: Vec<(Expr, BTreeMap<ValPath, ConstraintValue>, Type)>,
    pub globals_names: HashMap<String, ValPath>,
    pub type_decls: Vec<TypeDecl>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ValPath {
    Local(Vec<u16>),
    Capture(Vec<u16>),
    StaticVal(Vec<u16>),
    StaticFn(u16),
    Constructor,
}

pub struct TypeDecl {
    pub name: String,
    pub num_generics: u16,
    pub variants: Vec<(String,Type)>,
}

pub struct Closure {
    pub captures: Vec<(ValPath, Type)>,
    pub args: Vec<Type>,
    pub return_type: Type,
    pub dtree: DTree,
    pub branches: Vec<Expr>,
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

pub enum Expr {
    Literal(Literal),
    Bound(ValPath),
    Tuple(Vec<Expr>),

    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UnOp(UnOpcode, Box<Expr>),

    Closure(u16),
    Application(Box<Expr>, Box<Expr>),

    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),

    Error,
}