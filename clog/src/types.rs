use std::{
    cmp::max,
    collections::HashMap,
    fmt,
};
use crate::{
    error::Error,
};


/// Representation of a sum type
#[derive(Debug)]
pub struct TypeDecl<'input> {
    pub name: &'input str,
    
    /// number of generics
    pub num_generics: u16,

    #[cfg(debug_assertions)]
    pub variants: Vec<(&'input str, Type)>,

    #[cfg(not(debug_assertions))]
    pub variants: Vec<Type>
}

#[derive(Debug)]
pub enum ProtoType<'input> {
    Unit,
    Int, Bool, String,
    Function(Box<ProtoType<'input>>, Box<ProtoType<'input>>),
    Tuple(Vec<ProtoType<'input>>),
    Sum(&'input str, Box<ProtoType<'input>>),
    Generic(&'input str),
    /// Parse error
    Error(usize, usize),
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
    /// Sum type
    // a vector is used instead of a box type, because sum is frequently on a tuple type,
    // this optimizes the common case to 1 level of indirection instead of 2.
    Sum(u16, Vec<Type>),
    Generic(u16),
    Variable(u16),    // type variable only used for type-checking
}

impl Type {
    pub fn pretty_format(
        &self,result: &mut String,
        types: &Vec<TypeDecl>
    ){
        let call_self = |t: &Self, dst: &mut String| t.pretty_format(dst, types); 
        match *self {
            Type::Constructor { target, position } => {
                *result += types[target as usize].name;
                *result += types[target as usize].variants[position as usize - 1].0
            },
            Type::Sum(n, ref v) => {
                *result += types[n as usize].name;
                result.push_str("(");
                v[0].to_string_base(result, call_self);
                for t in v.iter().skip(1) {
                    result.push_str(", ");
                    t.to_string_base(result, call_self);
                }
                result.push_str(")")
            },
            _ => self.to_string_base(result, call_self),
        }
    }

    fn to_string_base<F: Fn(&Self, &mut String)>(&self, dst: &mut String, f: F) {
        match *self {
            Type::Unit => dst.push_str("()"),
            Type::Int => dst.push_str("int"),
            Type::Bool => dst.push_str("bool"),
            Type::String => dst.push_str("string"),
            Type::Constructor { target, position } => *dst += &format!("~{}::{}", target, position),
            Type::Function(ref from, ref to) => {
                if let Type::Function (..) = **from {
                    dst.push_str("(");
                    f(from.as_ref(), dst);
                    dst.push_str(") -> ");
                } else {
                    f(from.as_ref(), dst);
                   dst.push_str(" -> ");
                }
                f(to.as_ref(), dst)
            }
            Type::Tuple(ref v) => {
                dst.push_str("(");
                f(&v[0], dst);
                for t in v.iter().skip(1) {
                    dst.push_str(", ");
                    f(t, dst);
                }
                dst.push_str(")")
            }
            Type::Sum(n, ref v) => {
                *dst += &format!("~{}(", n);
                f(&v[0], dst);
                for t in v.iter().skip(1) {
                    dst.push_str(", ");
                    f(t, dst);
                }
                dst.push_str(")")
            },
            Type::Generic(n) => *dst += &format!("{}", ('a' as u16 + n) as u8 as char),
            Type::Variable(n) => *dst += &format!("{}", n),
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        fn to_string_base(t: &Type, dst: &mut String) {t.to_string_base(dst, to_string_base)}
        self.to_string_base(&mut s, to_string_base);
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal<'input> {
    Unit,
    Int(isize),
    Bool(bool),
    String(&'input str),
}

impl<'input> Literal<'input> {
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

impl<'input> ProtoType<'input> {
    pub fn to_type(
        self,
        type_map: &HashMap<&'input str, u16>, // map of type names -> index in types vector
        generics_map: &HashMap<&'input str, u16>,
    ) -> Result<Type,Error<'input>> {
        use self::ProtoType as P;
        use self::Type as T;
        match self {
            P::Unit => Ok(T::Unit),
            P::Int => Ok(T::Int),
            P::Bool => Ok(T::Bool),
            P::String => Ok(T::String),
            P::Tuple(v) => Ok(T::Tuple({
                let mut u = Vec::new();
                for t in v.into_iter() {
                    u.push(t.to_type(type_map, generics_map)?);
                }
                u            })),
            P::Function(from, to) => Ok(T::Function(
                Box::new(from.to_type(type_map, generics_map)?),
                Box::new(to.to_type(type_map, generics_map)?),
            )),
            P::Generic(name) => match generics_map.get(&name) {
                Some(&n) => Ok(T::Generic(n)),
                None => Err(Error::NameNotFound(name)),
            },
            P::Sum(name, t) => match type_map.get(&name) {
                Some(&n) => if let P::Tuple(v) = *t {
                    Ok(T::Sum(n, {
                        let mut u = Vec::new();
                        for t in v.into_iter() {
                            u.push(t.to_type(type_map, generics_map)?);
                        }
                        u
                    }))
                } else {
                    Ok(T::Sum(n, vec![t.to_type(type_map, generics_map)?]))
                }
                None => panic!("should be error type not defined"),
            },
            P::Error(..) => panic!("Parse Error not supposed to be propagated"),
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