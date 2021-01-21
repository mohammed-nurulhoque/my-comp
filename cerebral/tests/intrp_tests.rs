#[cfg(test)]
mod test {
    use {
        clog::{
            grammar, 
            type_check,
            parse,
        },
        cerebral::interpret,
        std::{
            fs::File,
            io::prelude::*,
        },
    };
    fn test_simple() {
        let mut f = File::open("tests/simple.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let contents = parse::uncomment(&mut contents);
        let result = parse::parse(&contents).unwrap();
        let module = type_check::ast2imper_ast(result).unwrap();
        let mut ctx = interpret::Context::new(&module);
        ctx.eval_toplevel();
    }

    fn test_fact() {
        fn fact(n: u32) -> u32 {
            if n == 0 { 1 } else { n * fact(n-1) }
        }
        let mut f = File::open("tests/fact.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let contents = parse::uncomment(&mut contents);
        let result = parse::parse(&contents).unwrap();
        let module = type_check::ast2imper_ast(result).unwrap();
        let mut ctx = interpret::Context::new(&module);
        ctx.eval_toplevel();
    }

    #[test]
    fn test_list() {
        let mut f = File::open("tests/list.mal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        let contents = parse::uncomment(&mut contents);
        let result = parse::parse(&contents).unwrap();
        let module = type_check::ast2imper_ast(result).unwrap();
        let mut ctx = interpret::Context::new(&module);
        ctx.eval_toplevel();
    }
}