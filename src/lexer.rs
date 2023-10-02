use std::str::Chars;
use crate::setbuilder::SetBuilder;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(isize),
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Range,
    Pipe,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Power,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    In,
    End
}

pub struct Lexer {
    current: usize,
    src: String
}

// TODO: remove
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

impl Lexer {
    pub fn new(src: String) -> Lexer {
        Lexer{current: 0, src: src}
    }

    fn next(&mut self) -> Option<char> {
        let char = self.src.chars().nth(self.current);
        self.current += 1;
        char
    }

    fn peek(&mut self) -> Option<char> {
        self.src.chars().nth(self.current)
    }

    fn peek_ahead(&mut self, offset: usize) -> Option<char> {
        self.src.chars().nth(self.current + offset)
    }

    fn throw_error(&mut self, message: &str) {
        println!("Error: {}", message);
    }

    pub fn lex(&mut self) -> Option<Vec<Token>> {
        let mut tokens = vec![];

        while let Some(cur_char) = self.next() {
            match cur_char {
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                '|' => tokens.push(Token::Pipe),
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Subtract),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '%' => tokens.push(Token::Modulus),
                '^' => tokens.push(Token::Power),
                '=' => {
                    if self.next() == Some('=') {
                        tokens.push(Token::Equal);
                    }
                    else {
                        self.throw_error("Unexpected operator. Did you mean \"==\"?");
                        return None;
                    }
                },
                '<' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::LessEqual);
                        self.next();
                    }
                    else if self.peek() == Some(' ') || self.peek().unwrap().is_alphanumeric() {
                        tokens.push(Token::Less);
                    }
                    else {
                        self.throw_error("Unexpected operator. Did you mean \"<=\"?");
                        return None;
                    }
                },
                '>' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::GreaterEqual);
                        self.next();
                    }
                    else if self.peek() == Some(' ') || self.peek().unwrap().is_alphanumeric() {
                        println!("{:?}", self.peek().unwrap());
                        tokens.push(Token::Greater);
                    }
                    else {
                        self.throw_error("Unexpected operator. Did you mean \">=\"?");
                        return None;
                    }
                },
                '.' => {
                    if self.peek() == Some('.') {
                        tokens.push(Token::Range);
                        self.next();
                    }
                },
                '0'..='9' => {
                    let mut number: String = cur_char.to_string();
                    while let Some(cur_char) = self.peek() {
                        if cur_char.is_digit(10) {
                            number += cur_char.to_string().as_str();
                            self.next();
                        }
                        else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                },
                'i' => {
                    if self.peek() == Some('n') {
                        if self.peek_ahead(1) == Some(' ') {
                            tokens.push(Token::In);
                            self.next();
                            self.next();
                        }
                    }
                },
                ' ' => continue,
                _ => {
                    let mut identifier = cur_char.to_string();
                    while let Some(cur_char) = self.peek() {
                        if cur_char.is_alphanumeric() {
                            identifier += cur_char.to_string().as_str();
                            self.next();
                        }
                        else {
                            break;
                        }
                    }
                    tokens.push(Token::Identifier(identifier));
                }
            }
        }
        tokens.push(Token::End);

        println!("{:?}", tokens);

        Some(tokens)
    }
}