use std::rc::Rc;

#[derive(Debug)]
pub enum ProtoType {
    Unit,
    Int, Bool, String,
    Function(Rc<ProtoType>, Rc<ProtoType>),
    Tuple(Vec<ProtoType>),
    Sum(String, Rc<ProtoType>),
    Generic(String),
}

pub enum Type {
    Unit,
    Int, Bool, String,
    Constructor(Rc<Type>, usize),
    Function(Rc<Type>, Rc<Type>),
    Tuple(Vec<Type>),
    Sum(usize, Rc<Type>),
    Generic(usize),
    Variable(usize),    // for type-checking
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Unit,
    Int(isize),
    Bool(bool),
    String(String),
}

impl Literal {
    pub fn get_type(&self) -> Type {
        match *self {
            Literal::Unit      => Type::Unit,
            Literal::Int(_)    => Type::Int,
            Literal::Bool(_)   => Type::Bool,
            Literal::String(_) => Type::String,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Greater,
    Less,
    GreaterEq,
    LessEq,

    Equal,
    NotEq,

    And,
    Or,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum UnOpcode {
    Minus,
    Not,
}