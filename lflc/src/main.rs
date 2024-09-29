use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//mod parse;
mod lex; // Temporary

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file passed.");
    } else {
        let path = Path::new(&args[1]);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(reason) => {
                eprintln!("Couldn't open file {}: {}", display, reason);
                return;
            },
            Ok(result) => result
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(reason) => {
                eprintln!("Couldn't read file {}: {}", display, reason);
                return;
            },
            Ok(_) => ()
        }

        //let ast = parse(&contents);
        println!("{:?}", lex::Lexer::new(&contents).lex());
    }
}