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
        let mut f = File::open("tests/foobar.maal").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("Cannot read file");
        println!("{:?}", grammar::ProgramParser::new().parse(&contents).unwrap());
    }
}