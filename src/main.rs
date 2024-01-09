#![feature(fs_try_exists)]

use std::process::Command;
use std::path::Path;
use std::env;

use codegenerator::{CodeGenerator, CGenerator};
use parser::Parser;
use lexer::Lexer;

mod setbuilder;
mod codegenerator;
mod parser;
mod lexer;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No input file provided");
        std::process::exit(1);
    }

    let input_filename = &args[1];
    let mut output_filename = input_filename;
    if args.len() > 2 {
        output_filename = &args[2];
    }
    let output_path = Path::new(output_filename.as_str());

    let src = std::fs::read_to_string(input_filename.as_str());
    if src.is_err() {
        eprintln!("Couldn't open input file");
        std::process::exit(1);
    }

    let mut lexer = Lexer::new(src.unwrap());
    let tokens = lexer.lex().unwrap();

    // Namessssss
    let mut parser = Parser::new(tokens);
    let ast = parser::parse(&mut parser);

    let mut code_generator: CGenerator = CodeGenerator::new(ast);

    // Compile generated C code using GCC
    _ = std::fs::write("tmp.c", code_generator.generate());
    // Rust.
    if Command::new("gcc")
        .args(["tmp.c", "-o", format!("{}.exe", output_path.file_stem().unwrap().to_str().unwrap()).as_str()])
        .status()
        .unwrap()
        .success() {
            _ = std::fs::remove_file("tmp.c");
    }
}