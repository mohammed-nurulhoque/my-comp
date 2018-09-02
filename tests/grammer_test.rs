extern crate mylang;

#[cfg(test)]
mod test {
    use {
        mylang::grammar, 
        std::{
            fs::File,
            io::prelude::*,
        },
    };
    #[test]
    fn test () {
        //use mylang::{ast::*, types::*};
        let parser = grammar::ProgramParser::new();
        let mut f = File::open("tests/foobar.maal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let result = parser.parse(&contents);
        assert!(result.is_ok());
        let _result = result.unwrap();

    }
}