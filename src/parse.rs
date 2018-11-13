use regex::Regex;
use std::{
    io::prelude::*,
    fs::File,
};
use crate::{
    error::Error,
};

pub fn read_file(filename: &str) -> Result<String, Error<'static>> {
    let mut f = File::open(filename)?;
    let mut cont = String::new();
    f.read_to_string(&mut cont)?;
    let uncommented = uncomment(&cont);
    Ok(uncommented)
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