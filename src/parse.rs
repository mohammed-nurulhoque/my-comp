use regex::Regex;
use std::{
    io::prelude::*,
    fs::File,
};
use crate::{
    grammar::ProgramParser,
    ast::Binding,
    error::Error,
};

pub fn parse_file(parser: &ProgramParser, filename: &str) -> Result<Vec<Binding>, Error> {
    let mut f = File::open(filename)?;
    let mut cont = String::new();
    f.read_to_string(&mut cont)?;
    let uncommented = uncomment(&cont);
    match parser.parse(&uncommented) {
        Ok(result) => Ok(result),
        Err(_) => Err(Error::ParseErr),
    }
}

pub fn uncomment(src: &str) -> String {
    let re = Regex::new(r"(//.*)|(/\*(.|\n)*?\*/)").unwrap();
    let mut dst = String::new();
    let mut offset = 0;
    for mat in re.find_iter(src) {
        dst += &src[offset..mat.start()];
        offset = mat.end();
    }
    dst
}