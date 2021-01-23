#![feature(box_syntax)]

use std::env;
use std::iter::FromIterator;
use std::collections::{
    HashMap
};
use std::fs::File;
use std::io::{
    stdin,
    prelude::*
};

use clog::{
    parse,
    type_check,
    imper_ast::ValPath,
    types::Type
};

mod interpret;

fn main() {
    let mut args = env::args();
    args.next();
    let input_file = args.next().expect("No input file given");
    let mut f = File::open(input_file).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Cannot read file");
    let contents = parse::uncomment(&mut contents);
    let bindings = parse::parse(&contents).unwrap();
    
    let mut ctx = type_check::TypingContext::new();
    let stl_map = HashMap::from_iter(vec![
        ("print", (ValPath::Imported("print"), 
            Type::Function(Box::new(Type::String), Box::new(Type::Unit)))),
        ("i2str", (ValPath::Imported("i2str"),
            Type::Function(Box::new(Type::Int), Box::new(Type::String)))),
    ]);
    ctx.add_imports(stl_map);

    for b in bindings {
        ctx.add_binding(b).expect("Failed to type check");
    }
    let module = ctx.export().unwrap();
    let mut ctx = interpret::Context::new(&module);
    ctx.eval_toplevel();
}

fn repl() {
    let mut s = String::new();
    let ctx = type_check::TypingContext::new();
    for line in stdin().lock().lines() {
        s += line.unwrap().trim_right();
        if s.chars().last().unwrap() == ';' {
            s.pop();
            let contents = parse::uncomment(&mut s);
            let result = parse::parse(&contents).unwrap();
            // let module = type_check::ast2imper_ast(result).unwrap();
            // let mut ctx = interpret::Context::new(&module);
            // ctx.eval_toplevel();
        }
    }
}