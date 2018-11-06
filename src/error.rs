use crate::types::Type;

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