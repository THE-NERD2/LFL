use ast::*;
use lex::*;

fn parse(code: &str) -> Module {
    let tokens = Lexer::new(code).lex();
    
}