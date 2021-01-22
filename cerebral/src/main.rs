#![feature(box_syntax)]

use std::env;
use std::fs::File;
use std::io::{
    stdin,
    prelude::*
};

use clog::{
    parse,
    type_check,
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
    let result = parse::parse(&contents).unwrap();
    let module = type_check::ast2imper_ast(result).unwrap();
    let mut ctx = interpret::Context::new(&module);
    ctx.eval_toplevel();
}

fn repl() {
    let mut s = String::new();
    for line in stdin().lock().lines() {
        s += line.unwrap().trim_right();
        if s.chars().last().unwrap() == ';' {
            s.pop();
            let contents = parse::uncomment(&mut s);
            let result = parse::parse(&contents).unwrap();
            let module = type_check::ast2imper_ast(result).unwrap();
            let mut ctx = interpret::Context::new(&module);
            ctx.eval_toplevel();
        }
    }
}