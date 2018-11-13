use std::{
    cmp::max,
    collections::HashMap,
    fmt,
};

#[derive(Debug)]
pub enum ProtoType {
    Unit,
    Int, Bool, String,
    Function(Box<ProtoType>, Box<ProtoType>),
    Tuple(Vec<ProtoType>),
    Sum(String, Box<ProtoType>),
    Generic(String),
}

#[derive(Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Int, Bool, String,
    Constructor {
        /// target type in a global types vector
        target: u16,
        /// position among the constructors of the same type starting from 1
        /// because path.0 is reserved for tag in ValuePath
        position: u16,
    },
    Function(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Sum(u16, Vec<Type>),
    Generic(u16),
    Variable(u16),    // for type-checking
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Unit => write!(f, "()"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Constructor { target, position }=> write!(f, "~{}::{}", target, position),
            Type::Function(ref from, ref to) => {
                if let Type::Function (..) = **from {
                    write!(f, "(")?;
                    from.as_ref().fmt(f)?;
                    write!(f, ") -> ")?;
                } else {
                    from.as_ref().fmt(f)?;
                    write!(f, " -> ")?;
                }
                to.as_ref().fmt(f)
            }
            Type::Tuple(ref v) => {
                write!(f, "(")?;
                v[0].fmt(f)?;
                for t in v.iter().skip(1) {
                    write!(f, ", ")?;
                    t.fmt(f)?;
                }
                write!(f, ")")
            }
            Type::Sum(n, ref t) => {
                write!(f, "~{}(", n)?;
                t.fmt(f)?;
                write!(f, ")")
            },
            Type::Generic(n) => write!(f, "{}", ('a' as u16 + n) as u8 as char),
            Type::Variable(n) => write!(f, "{}", n),
        }
    }
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

impl ProtoType {
    pub fn to_type(
        self,
        type_map: &HashMap<String, u16>, // map of type names -> index in types vector
        generics_map: &HashMap<String, u16>,
    ) -> Type {
        use self::ProtoType as P;
        use self::Type as T;
        match self {
            P::Unit => T::Unit,
            P::Int => T::Int,
            P::Bool => T::Bool,
            P::String => T::String,
            P::Tuple(v) => T::Tuple(v.into_iter().map(|t| t.to_type(type_map, generics_map)).collect()),
            P::Function(from, to) => T::Function(
                Box::new(from.to_type(type_map, generics_map)),
                Box::new(to.to_type(type_map, generics_map)),
            ),
            P::Generic(name) => match generics_map.get(&name) {
                Some(&n) => T::Generic(n),
                None => panic!("Error, generic not found"),
            },
            P::Sum(name, t) => match type_map.get(&name) {
                Some(&n) => if let P::Tuple(v) = *t {
                    T::Sum(n, v.into_iter().map(|t| t.to_type(type_map, generics_map)).collect())
                } else {
                    T::Sum(n, vec![t.to_type(type_map, generics_map)])
                }
                None => panic!("should be error type not defined"),
            },
        }
    }
}

impl Type {
    /// instantiate a type by substituting generics with variables
    /// starting from var, 
    /// Generic(n) => Variable(var + n)
    /// ### RETURNS
    /// instantiated type and next free variable
    pub fn instantiate(&self, var: u16) -> (Type, u16) {
        match *self {
            Type::Unit | Type::Int | Type::Bool | Type::String | Type::Constructor {..} | Type::Variable(_) => (self.clone(), var),
            Type::Function(ref from, ref to) => {
                let (from, next) = from.instantiate(var);
                let (to, nnext) = to.instantiate(var);
                (
                    Type::Function(Box::new(from), Box::new(to)),
                    max(next, nnext),
                )
            },
            Type::Generic(n) => (Type::Variable(var + n), var + n + 1),
            Type::Sum(n, ref v) => {
                let (v, next): (Vec<Type>, Vec<u16>) = v.iter().map(|t| t.instantiate(var)).unzip();
                let next = next.into_iter().fold(var, |acc, elem| max(acc, elem));
                (Type::Sum(n, v), next)
            }
            Type::Tuple(ref v) => {
                let (v, next): (Vec<Type>, Vec<u16>) = v.iter().map(|t| t.instantiate(var)).unzip();
                let next = next.into_iter().fold(var, |acc, elem| max(acc, elem));
                (Type::Tuple(v), next)
            }
        }
    }

    // convert variables to generics
    fn generalize(&mut self, map: &mut HashMap<u16, u16>) {
        match *self {
            Type::Int | Type::Bool | Type::String | Type::Unit | Type::Constructor {..} => (),
            Type::Variable(n) => {
                match map.get(&n) {
                    Some(&m) => *self = Type::Generic(m),
                    None => {
                        let len = map.len() as u16;
                        map.insert(n, len);
                        *self = Type::Generic(len)
                    }
                }
            }
            Type::Function(ref mut from, ref mut to) => {
                from.generalize(map);
                to.generalize(map);
            }
            Type::Tuple(ref mut v) |  Type::Sum(_, ref mut v) => {
                for t in v {
                    t.generalize(map);
                }
            }
            Type::Generic(_) => panic!("Generic not expected in generalize"), // maybe remove
        }
    }

    pub fn generalize_type(&mut self) {
        self.generalize(&mut HashMap::new())
    }
}