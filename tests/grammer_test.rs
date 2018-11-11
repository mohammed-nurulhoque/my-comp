#[cfg(test)]
mod test {
    use {
        mylang::grammar, 
        mylang::type_check,
        std::{
            fs::File,
            io::prelude::*,
        },
    };
    #[test]
    fn test_parser () {
        let parser = grammar::ProgramParser::new();
        let mut f = File::open("tests/test.test").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let result = parser.parse(&contents);
        assert!(result.is_ok());
        let _result = result.unwrap();
    }

    #[test]
    fn test_reduce () {
        let parser = grammar::ProgramParser::new();
        let mut f = File::open("tests/reduce.maal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let result = parser.parse(&contents).unwrap();
        type_check::ast2imper_ast(result).unwrap();
    }
}