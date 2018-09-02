#[derive(Debug)]
pub enum ProtoType {
    Unit,
    Int, Bool, String,
    Function(Box<ProtoType>, Box<ProtoType>),
    Tuple(Vec<ProtoType>),
    Sum(String, Box<ProtoType>),
    Generic(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Int, Bool, String,
    Constructor {
        /// the input to the constructor
        arg: Box<Type>,
        /// target type in a global types vector
        target: u16,
        /// position among the constructors of the same type
        position: u16,
    },
    Function(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Sum(u16, Box<Type>),
    Generic(u16),
    Variable(u16),    // for type-checking
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