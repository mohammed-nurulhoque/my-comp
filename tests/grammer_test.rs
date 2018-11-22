use mylang::parse::uncomment;
#[cfg(test)]
mod test {
    use {
        mylang::{
            grammar, 
            type_check,
            parse,
        },
        std::{
            fs::File,
            io::prelude::*,
        },
    };
    #[test]
    fn test_parser () {
        use super::*;
        let mut f = File::open("tests/parsed.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let result = parse::parse(&contents);
        assert!(result.is_ok());
        let _result = result.unwrap();
    }

    #[test]
    fn test_reduce () {
        use super::*;
        let mut f = File::open("tests/type_checked.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let contents = uncomment(&mut contents);
        let result = parse::parse(&contents).unwrap();
        type_check::ast2imper_ast(result).unwrap();
    }

    #[test]
    fn test_err() {
        use super::*;
        let mut f = File::open("tests/parse_err.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let contents = uncomment(&mut contents);
        let result = parse::parse(&contents);
        println!("{:?}", result)
    }
}