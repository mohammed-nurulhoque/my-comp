//! Author Mohammed Nurul Hoque, Aug 16, 2020
//! An interpreter for clog

use std::{
    rc::Rc,
};

use crate::{
    imper_ast::{
        Module,
        ValPath,
        Expr,
    },
    types::{
        Literal,
        UnOpcode,
        BinOpcode
    },
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::type_check::ast2imper_ast;
    use crate::parse::parse;

    macro_rules! expr {
        (Int($e:expr)) => {
            Expr::Literal(Literal::Int($e))
        };
        (+ ($($e1:tt)+), ($($e2:tt)+)) => {
            Expr::BinOp(box(expr!($($e1)+)), BinOpcode::Add, box(expr!($($e2)+)))
        };
        (- ($($e:tt)+)) => {
            Expr::UnOp(UnOpcode::Minus, box(expr!($($e)+)))
        };
    }
    #[test]
    fn interpret_simple_expr() {
        let e = expr!(+ (Int(1)), (-(Int(3))));
        let module = ast2imper_ast(vec![]).unwrap();
        let ctx = Context::new(&module);
        println!("{:?}", ctx.eval_exp(&e).unwrap());
    }

    fn interpret_function () {

    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Unit,
    Int(isize),
    Bool(bool),
    String(String),
    /// nth type's mth constructor applied to value
    SumVar(u16, u16, Rc<Value>),
    Tuple(Vec<Rc<Value>>),
    /// nth function from context, caputuring list of values and 
    /// currying partially applied with second list of values
    Closure(u16, Vec<Rc<Value>>, Vec<Rc<Value>>),
}

#[derive(Debug)]
pub enum IntrpErr {
    TypeMismatch,
    InvalidPath,
    NonExhaustivePattern,
}

pub struct Context<'a,'input> {
    module: &'a Module<'input>,
    locals: Vec<Rc<Value>>,
    captures: Vec<Rc<Value>>,
}


impl<'a,'input> Context<'a,'input> {
    pub fn new(module: &'a Module<'input>) -> Self {
        Context {
            module,
            locals: vec![],
            captures: vec![]
        }
    }

    fn resolve(&self, path: &ValPath) -> Result<Rc<Value>, IntrpErr> {
        panic!("Unimplemented")
    }

    pub fn eval_exp(&self, expr: &Expr) -> Result<Rc<Value>, IntrpErr> {
        println!("{:?}", &expr);
        match expr {
            &Expr::Literal(Literal::Unit) => Ok(Rc::new(Value::Unit)),
            &Expr::Literal(Literal::Int(n)) => Ok(Rc::new(Value::Int(n))),
            &Expr::Literal(Literal::Bool(p)) => Ok(Rc::new(Value::Bool(p))),
            &Expr::Literal(Literal::String(s)) => Ok(Rc::new(Value::String(s.to_owned()))),
            &Expr::Bound(ref path) => self.resolve(path),
            &Expr::Tuple(ref v) => Ok(Rc::new(Value::Tuple({
                let mut v2 = vec!();
                for e in v {
                    v2.push(self.eval_exp(e)?)
                }
                v2
            }))),

            &Expr::UnOp(op, ref e) => match (op, &*self.eval_exp(e)?) {
                (UnOpcode::Not, &Value::Bool(p)) => Ok(Rc::new(Value::Bool(!p))),
                (UnOpcode::Minus, &Value::Int(n)) => Ok(Rc::new(Value::Int(-n))),
                _ => Err(IntrpErr::TypeMismatch),
            },

            &Expr::BinOp(ref e1, op, ref e2) => 
                match (&*self.eval_exp(e1)?, &*self.eval_exp(e2)?) {
                (Value::Int(n), Value::Int(m)) => match op {
                    BinOpcode::Add => Ok(Rc::new(Value::Int(n+m))),
                    BinOpcode::Sub => Ok(Rc::new(Value::Int(n-m))),
                    BinOpcode::Mul => Ok(Rc::new(Value::Int(n*m))),
                    BinOpcode::Div => Ok(Rc::new(Value::Int(n/m))),
                    BinOpcode::Mod => Ok(Rc::new(Value::Int(n % m))),
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(n == m))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(n != m))),
                    BinOpcode::Greater => Ok(Rc::new(Value::Bool(n > m))),
                    BinOpcode::GreaterEq => Ok(Rc::new(Value::Bool(n >= m))),
                    BinOpcode::Less => Ok(Rc::new(Value::Bool(n < m))),
                    BinOpcode::LessEq => Ok(Rc::new(Value::Bool(n <= m))),
                    _ => Err(IntrpErr::TypeMismatch),
                },
                (&Value::Bool(p), &Value::Bool(q)) => match op {
                    BinOpcode::And => Ok(Rc::new(Value::Bool(p && q))),
                    BinOpcode::Or => Ok(Rc::new(Value::Bool(p || q))),
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(p == q))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(p != q))),
                    _ => Err(IntrpErr::TypeMismatch),
                },
                (Value::Unit, Value::Unit) => match op {
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(true))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(false))),
                    _ => Err(IntrpErr::TypeMismatch),
                },
                (Value::String(s1), Value::String(s2)) => match op {
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(s1 == s2))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(s1 != s2))),
                    _ => Err(IntrpErr::TypeMismatch),
                },
                (Value::SumVar(n1, m1, v1), Value::SumVar(n2, m2, v2)) => match op {
                    _ => panic!("fail")
                    //BinOpcode::Equal => Ok(Value::Bool(n1 == n2 && m1 == m2 && 
                },
                (Value::Tuple(v1), Value::Tuple(v2)) => match op {
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(v1 == v2))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(v1 != v2))),
                    _ => Err(IntrpErr::TypeMismatch),
                }
                (c1 @ Value::Closure(..), c2 @ Value::Closure(..)) => match op {
                    BinOpcode::Equal => Ok(Rc::new(Value::Bool(c1 == c2))),
                    BinOpcode::NotEq => Ok(Rc::new(Value::Bool(c1 != c2))),
                    _ => Err(IntrpErr::TypeMismatch),
                }
                _ => Err(IntrpErr::TypeMismatch),
            },
            &Expr::Closure(n) => {
                
                Ok(Rc::new(Value::Closure(n, self.gen_captures(n)?, vec![])))
            },
            &Expr::Application(ref e1, ref e2) => {
                let closure = self.eval_exp(e1)?;
                match *closure {
                    Value::Closure(n, ref cap, ref cur) => {
                        let mut cur = cur.clone();
                        cur.push(self.eval_exp(e2)?);
                        if self.module.closures[n as usize].args.len() < cur.len() {
                            Ok(Rc::new(Value::Closure(n, cap.clone(), cur)))
                        } else {
                            panic!("")
                        }
                    },
                    _ => Err(IntrpErr::TypeMismatch)
                }
            },
            &Expr::SumVal { target, position, ref value } => {
                let value = self.eval_exp(value)?;
                Ok(Rc::new(Value::SumVar(target, position, value)))
            },
            &Expr::Conditional(ref cond, ref e1, ref e2) => {
                match *self.eval_exp(cond)? {
                    Value::Bool(true) => self.eval_exp(e1),
                    Value::Bool(false) => self.eval_exp(e2),
                    _ => Err(IntrpErr::TypeMismatch),
                }
            },
            &Expr::Error => panic!("Error"),
        }
    }

    fn call_fn(
        &self, n: u16, captures: Vec<Rc<Value>>, locals: Vec<Rc<Value>>
    ) -> Result<Rc<Value>, IntrpErr> {
        let func = &self.module.closures[n as usize];
        let matched_arm = func.dtree.match_tree(&locals)?;
        let ctx = Context {
            module: self.module,
            captures,
            locals
        };
        ctx.eval_exp(&func.branches[matched_arm as usize]);
        panic!("")
    }

    /// Generate vector of captured value for nth 
    fn gen_captures(&self, n: u16) -> Result<Vec<Rc<Value>>, IntrpErr> {
        let closure = &self.module.closures[n as usize];
        let mut captures = (0..(closure.captures.len())).map(|_| None).collect::<Vec<Option<Rc<Value>>>>();

        for (path, _) in &closure.captures {
            match path {
                ValPath::CaptureLocal(i, ref v) => {
                    captures[*i as usize].replace(pathvec_from_valvec(v, &self.locals)?);
                },
                ValPath::CaptureCaptured(i, j) => {
                    captures[*i as usize].replace(pathvec_from_valvec(&[*j], &self.captures)?);
                },
                _ => return Err(IntrpErr::InvalidPath),
            }
        }

        Ok(captures.into_iter().map(|opt| opt.unwrap()).collect())
    }
}

pub fn pathvec_from_valvec(path: &[u16], vec: &Vec<Rc<Value>>) -> Result<Rc<Value>, IntrpErr> {
    fn pathvec_from_val(path: &[u16], val: &Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
        match path {
            [] => Ok(val.clone()),
            [n, tail @ ..] => {
                match **val {
                    Value::Tuple(ref v) => pathvec_from_val(tail, &v[*n as usize]),
                    Value::SumVar(ty_idx, con_idx, ref inner_val) => {
                        if *n == con_idx {
                            pathvec_from_val(tail, inner_val)
                        } else { Err(IntrpErr::InvalidPath)}
                    }
                    _ => Err(IntrpErr::InvalidPath),
                }
            }
        }
    }
    match path {
        [] => Err(IntrpErr::InvalidPath),
        [n, tail @ ..] => {
            pathvec_from_val(tail, &vec[*n as usize])
        }
    }
}