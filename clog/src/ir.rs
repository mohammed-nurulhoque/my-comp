//! This module specifies the format of the intermediate representaion. It
//! represents the program as an array of modules, where each module is a
//! list of globals (one for each toplevel declaration).
//!

// TODO: currently in the process of migrating imper_ast to this
// TODO: replace with imper_ast::TypeDecl
// TODO: make  memaddr's use ValPath instead of tmps

use crate::{
    imper_ast::ValPath,
    types::{ Type, BinOpcode, UnOpcode, Literal }
};
use peterson::graph::Graph;


struct TypeDecl<'input, 'types> {
    _empty: ::std::marker::PhantomData<&'input &'types ()>
}


struct Module<'input,'types> {
    /// type definitions in this module
    ty_def: Vec<TypeDecl<'input,'types>>,
    /// table of types, the remaining fields' types are references to this vec
    ty_table: Vec<Type>,
    /// layout of global memory of this module
    gl_layout: Vec<&'types Type>,
    /// vec of all functions/closures
    procedures: Vec<Procedure<'input,'types>>,
    /// name bindings in this module
    bindings: Vec<(&'input str, ValPath)>,
    /// initializes values of bindings
    init_fn: Procedure<'input,'types>,
}


struct Procedure<'input,'types> {
    /// labeled graph where statemnts are edges.
    control_flow: Graph<u32, Statement<'input, 'types>>,
    in_ty: Vec<Type>,
    out_ty: Type,
    num_generics: u16,
}

pub type tmpIdx = u16;

/// a simple statement.
enum Statement<'input,'types> {
    /// fetch from memaddr to temporary
    Fetch(tmpIdx, MemAddr<'types>),
    /// assign value to the temporary
    Assign(tmpIdx, Value<'input>),
    /// store value of temporary to memaddr
    Store(tmpIdx, MemAddr<'types>),
    /// 
    Call(u16, u16),
    Conditional(u16),
    /// return the value of nth temporary from procedure
    Return(tmpIdx)
}

impl<'input, 'types> peterson::graph::Edge for Statement<'input, 'types> {}

enum MemAddr<'types> {
    /// address of the closure struct of the current function if any. 
    /// Passed as one of the args to function. At runtime, it has
    /// the representation
    /// struct Frame {
    ///     /// A pointer to the entry point of the current procedure
    ///     startPtr: usize,
    ///     args: (arg1_t, arg2_t, ...)
    ///     captures: (cptr1_t, cptr_2, ...)
    /// }
    SelfFrame,

    /// nth value on the stack (u16 is not a tmp but the actual position)
    Stack(u16),

    /// literal address in tmp variable
    Numeric(tmpIdx),

    /// offset from start of an array
    ArrayOffset {
        elem_type: &'types Type,
        /// read array addr from tmp
        array_addr: tmpIdx,
        /// read offset (in #elems) from tmp
        offset: tmpIdx,
    },
    /// offset in a struct
    StructOffset {
        /// types of fields
        tuple_type: &'types [Type],
        /// target is nth field
        target: u16,
    },
}


enum Value<'input> {
    Literal(Literal<'input>),
    Variable(tmpIdx),
    Binop(BinOpcode, tmpIdx, tmpIdx),
    Unop(UnOpcode, tmpIdx),
    Construct(u16, u16, u16),
}