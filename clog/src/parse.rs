use regex::Regex;
use std::{
    fs::File,
};
use lalrpop_util::ParseError;
use crate::{
    ast::Binding,
    error::Error,
    grammar::ProgramParser,
};

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let s = "";
    }
}

pub fn parse<'input>(input: &'input str) -> Result<Vec<Binding<'input>>, Vec<usize>> {
    let parser = ProgramParser::new();
    let mut errors = Vec::new();
    if let Ok(ast) = parser.parse(&mut errors, input) {
        return Ok(ast)
    }
    let newstr = find_replace(input, r#""(\\.|[^"\\])*""#, "\"\"");
    let re = Regex::new(r"type |fn |rec fn").unwrap();
    let mut index = 0;
    while index < newstr.len() {
        match parser.parse(&mut errors, &newstr[index..]) {
            Ok(_) => return Err(errors),
            Err(ParseError::InvalidToken { location }) => {
                errors.push(index + location);
                if let Some(m) = re.find_at(&newstr, location) {
                    index = index + m.start();
                } else {
                    break;
                }
            }
            Err(_) => panic!("Parser should have catched this error"),
        };
    }
    Err(errors)
}

pub fn uncomment(src: &str) -> String {
    find_replace(src, r"(//.*)|(/\*(.|\n)*?\*/)", "")
}

fn find_replace(src: &str, re: &str, replace: &str) -> String {
    let re = Regex::new(re).unwrap();
    let mut dst = String::new();
    let mut offset = 0;
    for mat in re.find_iter(src) {
        dst += &src[offset..mat.start()];
        dst += replace;
        offset = mat.end();
    }
    dst += &src[offset..];
    dst
}