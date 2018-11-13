use std::{
    io::Error as ioErr,
    convert::From,
};
use crate::types::Type;


/// All errors from AST -> imperAST phase, not used yet!
#[derive(Debug)]
pub enum Error {
    IOErr(ioErr),
    ParseErr,
    TypeMismatch(Type, Type),
    ConstructorUnification,
    NameNotFound(String),
    MultBindPattern(String),
    ConstructorNotFound(String),
    NonConstAppPattern(String),
    TypeNotDefined(String),
    VariablePatsNum,
}

impl From<ioErr> for Error {
    fn from(e: ioErr) -> Self {
        Error::IOErr(e)
    }
}