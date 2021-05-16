//! Author Mohammed Nurul Hoque, Aug 16, 2020
//! An interpreter for clog

use std::{
    fmt,
    rc::Rc,
};

use clog::{
    dtree::DTree,
    imper_ast::{ConstraintValue, Expr, Module, ValPath},
    types::{BinOpcode, Literal, UnOpcode},
};

use crate::stdlib::std_call;

#[cfg(test)]
mod test {
    use super::*;
    use clog::parse::parse;
    use clog::type_check::ast2imper_ast;

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
        println!("{}", ctx.eval_exp(&e).unwrap());
    }

    #[test]
    fn interpret_function() {
        let prgrm = "let add = {m => {n => m + n}}";
        let parsed = parse(prgrm).unwrap();
        let compiled = ast2imper_ast(parsed).unwrap();
        let ctx = Context::new(&compiled);
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Unit,
    Int(isize),
    Bool(bool),
    String(String),
    // value of sum tag, not directly accessible
    Tag(u16),
    /// nth type's mth constructor applied to value
    SumVar(u16, u16, Rc<Value>),
    Tuple(Vec<Rc<Value>>),
    /// nth function from context, caputuring list of values and
    /// currying partially applied with second list of values
    Closure(u16, Vec<Rc<Value>>, Vec<Rc<Value>>),
    Constructor(u16, u16),
    Imported(&'static str),
}

#[derive(Debug)]
pub enum IntrpErr {
    TypeMismatch,
    InvalidPath,
    NonExhaustivePattern,
}

pub struct Context<'a, 'input> {
    module: &'a Module<'input>,
    statics: Vec<Rc<Value>>,
    locals: Vec<Rc<Value>>,
    captures: Vec<Rc<Value>>,
}

impl<'a, 'input> Context<'a, 'input> {
    pub fn new(module: &'a Module<'input>) -> Self {
        Context {
            module,
            statics: vec![],
            locals: vec![],
            captures: vec![],
        }
    }

    pub fn eval_toplevel(&mut self) {
        for (e, _, _) in &self.module.globals {
            let value = self.eval_exp(e).unwrap();
            // println!("{}", value.display(self.module));
            self.statics.push(value);
            // check satisfaction of constraints and print?
        }
    }

    fn resolve(&self, path: &ValPath) -> Result<Rc<Value>, IntrpErr> {
        match path {
            ValPath::Local(ref v) => pathvec_from_valvec(v, &self.locals),
            ValPath::StaticVal(ref v) => pathvec_from_valvec(v, &self.statics),
            ValPath::CaptureLocal(i, _) => pathvec_from_valvec(&[*i], &self.captures),
            ValPath::CaptureCaptured(i, _) => pathvec_from_valvec(&[*i], &self.captures),
            ValPath::Constructor(i, j) => Ok(Rc::new(Value::Constructor(*i, *j))),
            ValPath::Imported(s) => Ok(Rc::new(Value::Imported(s))),
        }
    }

    pub fn eval_exp(&self, expr: &Expr) -> Result<Rc<Value>, IntrpErr> {
        //println!["{:?}", &expr];
        match expr {
            &Expr::Literal(Literal::Unit) => Ok(Rc::new(Value::Unit)),
            &Expr::Literal(Literal::Int(n)) => Ok(Rc::new(Value::Int(n))),
            &Expr::Literal(Literal::Bool(p)) => Ok(Rc::new(Value::Bool(p))),
            &Expr::Literal(Literal::String(s)) => Ok(Rc::new(Value::String(s.to_owned()))),
            &Expr::Bound(ref path) => self.resolve(path),
            &Expr::Slice(ref e1, ref e2, ref e3) => {
                match (&*self.eval_exp(e1)?, &*self.eval_exp(e2)?, &*self.eval_exp(e3)?) {
                    (Value::String(s), Value::Int(a), Value::Int(b)) => {
                        Ok(Rc::new(Value::String(s.chars().skip(*a as usize).take((b-a) as usize).collect())))
                    },
                    _ => panic!("Runtime Type Error")
                }
            }
            &Expr::UnOp(op, ref e) => self.eval_unop(op, e),
            &Expr::BinOp(ref e1, op, ref e2) => self.eval_binop(e1, op, e2),
            &Expr::Closure(n) => Ok(Rc::new(Value::Closure(n, self.gen_captures(n)?, vec![]))),
            &Expr::Tuple(ref v) => self.eval_tuple(v),
            &Expr::Application(ref e1, ref e2) => self.eval_appl(e1, e2),
            &Expr::SumVal {
                target,
                position,
                ref value,
            } => Ok(Rc::new(Value::SumVar(
                target,
                position,
                self.eval_exp(value)?,
            ))),
            &Expr::Conditional(ref cond, ref e1, ref e2) => match *self.eval_exp(cond)? {
                Value::Bool(true) => self.eval_exp(e1),
                Value::Bool(false) => self.eval_exp(e2),
                _ => Err(IntrpErr::TypeMismatch),
            },
            &Expr::Error => panic!("Error"),
        }
    }

    fn eval_unop(&self, op: UnOpcode, e: &Expr) -> Result<Rc<Value>, IntrpErr> {
        match (op, &*self.eval_exp(e)?) {
            (UnOpcode::Not, &Value::Bool(p)) => Ok(Rc::new(Value::Bool(!p))),
            (UnOpcode::Minus, &Value::Int(n)) => Ok(Rc::new(Value::Int(-n))),
            _ => Err(IntrpErr::TypeMismatch),
        }
    }


    // XXX equality needs complete rewrite
    // XXX collapse 2-level match to 1
    fn eval_binop(&self, e1: &Expr, op: BinOpcode, e2: &Expr) -> Result<Rc<Value>, IntrpErr> {
        use BinOpcode::*;
        match (&*self.eval_exp(e1)?, &*self.eval_exp(e2)?) {
            (Value::String(s), Value::Int(n)) => match op {
                Index => Ok(Rc::new(Value::Int(s.chars().nth(*n as usize)
                            .expect(&format!("Index {} out of range for string \"{}\"", n, s)) as isize))),
                _ => Err(IntrpErr::TypeMismatch)
            }
            (Value::Int(n), Value::Int(m)) => match op {
                Add => Ok(Rc::new(Value::Int(n + m))),
                Sub => Ok(Rc::new(Value::Int(n - m))),
                Mul => Ok(Rc::new(Value::Int(n * m))),
                Div => Ok(Rc::new(Value::Int(n / m))),
                Mod => Ok(Rc::new(Value::Int(n % m))),
                Equal => Ok(Rc::new(Value::Bool(n == m))),
                NotEq => Ok(Rc::new(Value::Bool(n != m))),
                Greater => Ok(Rc::new(Value::Bool(n > m))),
                GreaterEq => Ok(Rc::new(Value::Bool(n >= m))),
                Less => Ok(Rc::new(Value::Bool(n < m))),
                LessEq => Ok(Rc::new(Value::Bool(n <= m))),
                _ => Err(IntrpErr::TypeMismatch),
            },
            (&Value::Bool(p), &Value::Bool(q)) => match op {
                And => Ok(Rc::new(Value::Bool(p && q))),
                Or => Ok(Rc::new(Value::Bool(p || q))),
                Equal => Ok(Rc::new(Value::Bool(p == q))),
                NotEq => Ok(Rc::new(Value::Bool(p != q))),
                _ => Err(IntrpErr::TypeMismatch),
            },
            (Value::Unit, Value::Unit) => match op {
                Equal => Ok(Rc::new(Value::Bool(true))),
                NotEq => Ok(Rc::new(Value::Bool(false))),
                _ => Err(IntrpErr::TypeMismatch),
            },
            (Value::String(s1), Value::String(s2)) => match op {
                Equal => Ok(Rc::new(Value::Bool(s1 == s2))),
                NotEq => Ok(Rc::new(Value::Bool(s1 != s2))),
                Concat => Ok(Rc::new(Value::String(s1.clone()+s2))),
                _ => Err(IntrpErr::TypeMismatch),
            },

            // NOTE: All equality operations below need reconsideration
            (Value::SumVar(..), Value::SumVar(..)) => match op {
                Equal | BinOpcode::NotEq => panic!("sum type equality"),
                _ => Err(IntrpErr::TypeMismatch),
            },
            (Value::Tuple(v1), Value::Tuple(v2)) => match op {
                Equal => Ok(Rc::new(Value::Bool(v1 == v2))),
                NotEq => Ok(Rc::new(Value::Bool(v1 != v2))),
                _ => Err(IntrpErr::TypeMismatch),
            },
            (c1 @ Value::Closure(..), c2 @ Value::Closure(..)) => match op {
                Equal => Ok(Rc::new(Value::Bool(c1 == c2))),
                NotEq => Ok(Rc::new(Value::Bool(c1 != c2))),
                _ => Err(IntrpErr::TypeMismatch),
            },
            _ => Err(IntrpErr::TypeMismatch),
        }
    }

    fn eval_tuple(&self, v: &Vec<Expr>) -> Result<Rc<Value>, IntrpErr> {
        Ok(Rc::new(Value::Tuple(
            v.iter()
                .map(|e| self.eval_exp(e))
                .collect::<Result<Vec<_>, _>>()?,
        )))
    }

    fn eval_appl(&self, e1: &Expr, e2: &Expr) -> Result<Rc<Value>, IntrpErr> {
        match *self.eval_exp(e1)? {
            Value::Closure(n, ref cap, ref cur) => {
                let mut cur = cur.clone();
                cur.push(self.eval_exp(e2)?);
                if cur.len() < self.module.closures[n as usize].args.len() {
                    Ok(Rc::new(Value::Closure(n, cap.clone(), cur)))
                } else {
                    self.call_fn(n, cap.clone(), cur)
                }
            }
            Value::Constructor(i, j) => {
                let e2 = self.eval_exp(e2)?;
                Ok(Rc::new(Value::SumVar(i, j, e2)))
            }
            Value::Imported(name) => {
                let e2 = self.eval_exp(e2)?;
                std_call(name, e2)
            }
            _ => Err(IntrpErr::TypeMismatch),
        }
    }

    fn call_fn(
        &self,
        n: u16,
        captures: Vec<Rc<Value>>,
        locals: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, IntrpErr> {
        let func = &self.module.closures[n as usize];
        let matched_arm = match_tree(&func.dtree, &locals)?;
        let ctx = Context {
            module: self.module,
            statics: self.statics.clone(),
            captures,
            locals,
        };
        ctx.eval_exp(&func.branches[matched_arm as usize])
    }

    /// Generate vector of captured value for nth closure
    fn gen_captures(&self, n: u16) -> Result<Vec<Rc<Value>>, IntrpErr> {
        let closure = &self.module.closures[n as usize];
        let mut captures = vec![None; closure.captures.len()];

        for (path, _) in &closure.captures {
            match path {
                ValPath::CaptureLocal(i, ref v) => {
                    captures[*i as usize].replace(pathvec_from_valvec(v, &self.locals)?);
                }
                ValPath::CaptureCaptured(i, j) => {
                    captures[*i as usize].replace(pathvec_from_valvec(&[*j], &self.captures)?);
                }
                _ => return Err(IntrpErr::InvalidPath),
            }
        }

        Ok(captures.into_iter().flatten().collect())
    }
}

pub fn pathvec_from_valvec(path: &[u16], valvec: &Vec<Rc<Value>>) -> Result<Rc<Value>, IntrpErr> {
    fn pathvec_from_val(path: &[u16], val: &Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
        match path {
            [] => Ok(val.clone()),
            [n, tail @ ..] => match **val {
                Value::Tuple(ref v) => pathvec_from_val(tail, &v[*n as usize]),
                Value::SumVar(_ty_idx, con_idx, ref inner_val) => {
                    if *n == con_idx {
                        pathvec_from_val(tail, inner_val)
                    } else if *n == 0 {
                        Ok(Rc::new(Value::Tag(con_idx - 1)))
                    } else {
                        Err(IntrpErr::InvalidPath)
                    }
                }
                _ => Err(IntrpErr::InvalidPath),
            },
        }
    }
    match path {
        [] => Err(IntrpErr::InvalidPath),
        [n, tail @ ..] => pathvec_from_val(tail, &valvec[*n as usize]),
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
            Value::Bool(true) => write!(f, "true"),
            Value::Bool(false) => write!(f, "false"),
            Value::Int(i) => write!(f, "{}", i),
            Value::String(s) => write!(f, "{}", s),
            Value::Tuple(v) => v
                .iter()
                .map(|x| {
                    x.fmt(f)?;
                    write!(f, ", ")
                })
                .collect(),
            Value::Closure(n, ..) => write!(f, "<closure {}>", n),
            Value::SumVar(n, m, val) => write!(f, "<type {}>::<variant {}>{}", n, m, val),
            Value::Tag(n) => write!(f, "<tag {}>", n),
            Value::Constructor(n, m) => write!(f, "<Constructor({}, {}", n, m),
            Value::Imported(s) => write!(f, "fn::{}", s),
        }
    }
}

// impl Value {
//     fn display<'input>(&self, module: &Module<'input>) -> String {
//         fn _display<'input>(v: &Value, module: &Module<'input>, s: &mut String) -> fmt::Result {
//             match v {
//                 Value::Unit => write!(s, "()"),
//                 Value::Bool(true) => write!(s, "true"),
//                 Value::Bool(false) => write!(s, "false"),
//                 Value::Int(i) => write!(s, "{}", i),
//                 Value::String(sval) => write!(s, "{}", sval),
//                 Value::Tuple(v) => {
//                     write!(s, "(")?;
//                     for i in 0..(v.len() - 1) {
//                         _display(&v[i], module, s).expect("format error!");
//                         write!(s, ", ")?;
//                     }
//                     _display(&v.last().expect("Empty Tuple!"), module, s).expect("format error!");
//                     write!(s, ")")
//                 }
//                 Value::Closure(n, ..) => {
//                     write!(s, "<closure ")?;
//                     let closure = &module.closures[*n as usize];
//                     for t in &closure.args {
//                         write!(s, "{:#?} -> ", t)?;
//                     }
//                     write!(s, "{:#?}>", closure.return_type)
//                 }
//                 Value::SumVar(n, m, val) => {
//                     write!(s, "{}::", module.type_decls[*n as usize].name)?;
//                     write!(
//                         s,
//                         "{} ",
//                         module.type_decls[*n as usize].variants[*m as usize - 1].0
//                     )?;
//                     _display(val, module, s)
//                 }
//                 Value::Tag(n) => write!(s, "<tag {}>", n),
//                 Value::Constructor(n, m) => write!(
//                     s,
//                     "{}::{}",
//                     module.type_decls[*n as usize].name,
//                     module.type_decls[*n as usize].variants[*m as usize].0
//                 ),
//                 Value::Imported(name) => write!(s, "fn::{}", name),
//             }
//         }
//         let mut s = String::new();
//         _display(self, module, &mut s).expect("format error!");
//         s
//     }
// }

pub fn match_tree(tree: &DTree, valvec: &Vec<Rc<Value>>) -> Result<u16, IntrpErr> {
    match tree {
        &DTree::Empty => Err(IntrpErr::NonExhaustivePattern),
        &DTree::Exit(m) => Ok(m),
        &DTree::Finite {
            value: ValPath::Local(ref v),
            ref branches,
        } => {
            let val = pathvec_from_valvec(v, valvec)?;
            match *val {
                Value::Bool(false) => match_tree(&branches[0], valvec),
                Value::Bool(true) => match_tree(&branches[1], valvec),
                // XXX: double check off by 1.
                Value::Tag(n) => match_tree(&branches[n as usize], valvec),
                _ => Err(IntrpErr::TypeMismatch),
            }
        }
        &DTree::Infinite {
            value: ValPath::Local(ref v),
            ref branches,
            ref default,
        } => match *pathvec_from_valvec(v, valvec)? {
            Value::Int(n) => match_tree(
                branches.get(&ConstraintValue::Int(n)).unwrap_or(default),
                valvec,
            ),
            Value::String(ref s) => match_tree(
                branches.get(&ConstraintValue::Str(s)).unwrap_or(default),
                valvec,
            ),
            _ => Err(IntrpErr::TypeMismatch),
        },
        _ => Err(IntrpErr::InvalidPath),
    }
}
