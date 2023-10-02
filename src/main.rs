use std::env;

mod setbuilder;
mod parser;
mod lexer;

use lexer::Lexer;
use lexer::Token;
use parser::Parser;

fn main() {
    // Get the filename from the command line
    // let filename: String = env::args().nth(1).expect("we are not gaming...");
    let src: String = std::fs::read_to_string("samples\\set1.sb").expect("my guy...");

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens.unwrap());
}