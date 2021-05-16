use std::{
    rc::Rc,
    collections::HashMap,
    io::{stdin, BufRead},
};
use clog::{
    types::Type,
    imper_ast::ValPath,
};
use crate::{
    interpret::{Value, IntrpErr}
};

static mut STL: Option<HashMap<&'static str, (Type, fn(Rc<Value>) -> Result<Rc<Value>, IntrpErr>)>> = None;

pub fn std_imports() -> HashMap<&'static str, (ValPath, Type)> {
    let mut stlmap = HashMap::new();
    stlmap.insert("print", 
        (   Type::Function(Box::new(Type::String), Box::new(Type::Unit)), 
            cn_print as fn(Rc<Value>) -> Result<Rc<Value>, IntrpErr>));
    stlmap.insert("i2str",
        (Type::Function(Box::new(Type::Int), Box::new(Type::String)), cn_i2s));
    stlmap.insert("readline", 
        (Type::Function(Box::new(Type::Unit), Box::new(Type::String)), cn_readline));   
    stlmap.insert("len", 
        (Type::Function(Box::new(Type::String), Box::new(Type::Int)), cn_len));   

    let mut map = HashMap::new();
    for (name, (t, _)) in &stlmap {
        map.insert(*name, (ValPath::Imported(name), t.clone()));
    }

    unsafe {  
        STL.get_or_insert(stlmap);
    }
    map
}

pub fn std_call(function: &str, value: Rc<Value>) -> Result<Rc<Value>,IntrpErr> {
    unsafe {
        STL.as_ref().unwrap()[function].1(value)
    }
}

fn cn_print(s: Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
    if let Value::String(ref s) = *s {
        print!("{}", s.replace("\\n", "\n"));
        Ok(Rc::new(Value::Unit))
    } else {
        panic!("Runtime type error")
    }
}

fn cn_i2s(n: Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
    if let Value::Int(i) = *n {
        Ok(Rc::new(Value::String(format!("{}", i))))
    } else {
        panic!("Runtime type error")
    }
}

fn cn_readline(_: Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
    let mut s = String::new();
    match stdin().lock().read_line(&mut s) {
        Ok(_) => Ok(Rc::new(Value::String(s))),
        Err(n) => panic!("IO ERROR [{}]", n),
    }
}

fn cn_len(s: Rc<Value>) -> Result<Rc<Value>, IntrpErr> {
    if let Value::String(ref s) = *s {
        Ok(Rc::new(Value::Int(s.len() as isize)))
    } else {
        panic!("Runtime type error")
    }
}