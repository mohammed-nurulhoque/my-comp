use std::{
    io::Error as ioErr,
    convert::From,
};
use crate::types::Type;


/// All errors from AST -> imperAST phase, not used yet!
#[derive(Debug)]
pub enum Error<'input> {
    IOErr(ioErr),
    ParseErr,
    TypeMismatch(Type, Type),
    ConstructorUnification,
    NameNotFound(&'input str),
    MultBindPattern(&'input str),
    ConstructorNotFound(&'input str),
    NonConstAppPattern(&'input str),
    TypeNotDefined(&'input str),
    VariablePatsNum,
}

impl<'input> From<ioErr> for Error<'input> {
    fn from(e: ioErr) -> Self {
        Error::IOErr(e)
    }
}