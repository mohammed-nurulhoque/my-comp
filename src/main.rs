extern crate mylang;
use mylang::grammar::{
    ExprParser,
};

fn main() {
    println!("{:?}", ExprParser::new().parse("1 + 2*3"));
}
