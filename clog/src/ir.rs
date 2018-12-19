use crate::{
    imper_ast::ValPath,
    types::{ Type, BinOpcode, UnOpcode, Literal }
};
use peterson::graph::Graph;

struct Module<'input,'types> {
    init_fn: Procedure<'input,'types>,
    procedures: Vec<Procedure<'input,'types>>,
    ty_table: Vec<Type>,
}

struct Procedure<'input,'types> {
    control_flow: Graph<u32, (),>,
    statements: Vec<Statement<'input,'types>>,
    in_ty: Vec<Type>,
    out_ty: Type,
    num_generics: u16,
}

enum Statement<'input,'types> {
    Fetch(u16, MemAddr<'types>),
    Assign(u16, Value<'input>),
    Store(u16, MemAddr<'types>),
    Call(u16, u16),
    Conditional(u16),
    Return(u16)
}

enum MemAddr<'types> {
    /// address of the closure struct of the current function if any. 
    /// Always at a determined position on the stack
    SelfFrame,
    /// A pointer to the entry point of the current procedure (inside SelfFrame and has same value
    /// but different types)
    StartPtr,

    /// nth value on the stack, the first ones are the curried argument in order
    Stack(u16),
    /// exact address in varibale
    Numeric(u16),

    /// offset from start of an array
    ArrayOffset {
        elem_type: &'types Type, 
        array_addr: u16, 
        offset: u16,
    },
    /// offset in a struct
    StructOffset {
        /// types of fields to skip, offset surpasses them
        tuple_type: &'types,
        target: Type,
    },
}

enum Value<'input> {
    Literal(Literal<'input>),
    Variable(u16),
    Binop(BinOpcode, u16, u16),
    Unop(UnOpcode, u16),
    Construct(u16, u16, u16),
}