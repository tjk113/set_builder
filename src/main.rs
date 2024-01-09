mod setbuilder;
mod codegenerator;
mod parser;
mod lexer;

use codegenerator::CodeGenerator;
use parser::Parser;
use lexer::Lexer;

fn main() {
    // Get the filename from the command line
    // let filename: String = env::args().nth(1).expect("we are not gaming...");
    let src: String = std::fs::read_to_string("samples\\set1.lc").expect("my guy...");

    let mut lexer = Lexer::new(src);
    let tokens = lexer.lex().unwrap();

    // Namessssss
    let mut parser = Parser::new(tokens);
    let ast = parser::parse(&mut parser);
    dbg!(ast);

    // let mut code_generator = CodeGenerator::new(parser::parse(&mut parser));
}