pub enum Binding {
    Type { name: String, vars: Vec<String>, variants: Vec<(String, Type)> },
    Value(Pattern, Expr),
    Function(String, Vec<(Pattern, Expr)>),
}

pub enum Type {
    Unit,
    Int, Bool, String,
    Function(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Sum(String, Box<Type>),
    Generic(String),
}

#[derive(Debug)]
pub enum Pattern {
    Wild,
    Literal(Literal),
    Bind(String),
    Tuple(Vec<Pattern>),
    SumVar { name: String, variant: String, field: Box<Pattern> },
}

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Bound(String),
    Tuple(Vec<Expr>),
    SumVar { name: String, variant: String, field: Box<Expr> },

    BinOp(Box<Expr>, BinOpcode, Box<Expr>),
    UnOp(UnOpcode, Box<Expr>),

    Function(Vec<(Pattern, Expr)>),
    Application(Box<Expr>, Box<Expr>),

    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    Unit,
    Int(isize),
    Bool(bool),
    String(String),
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