#![feature(nll)]
#![feature(slice_patterns)]
#![feature(box_syntax)]
//#![warn(missing_docs)]
pub mod ast;
pub mod grammar;
pub mod error;
mod types;
pub mod type_check;
pub mod imper_ast;
mod unify;
mod dtree;
mod namescope;
pub mod parse; // make private
pub mod interpret;