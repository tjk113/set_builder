use std::str::Chars;
use crate::setbuilder::SetBuilder;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(f32),
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
    NotEqual,
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

    fn handle_number(&mut self, tokens: &mut Vec<Token>, cur_char: char, negative: bool) {
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
        let mut final_number: f32 = number.parse().unwrap();
        // Make number negative if need be
        if negative {
            final_number = -final_number;
        }

        tokens.push(Token::Number(final_number));
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
                '-' => {
                    // Test to see if the next character is a number
                    let next_character = self.peek().unwrap();
                    for char_digit in '0'..='9' {
                        if next_character == char_digit {
                            self.handle_number(&mut tokens, cur_char, true);
                        }
                    }
                    // If it's not a negative number, it's
                    // just a regular subtraction symbol
                    tokens.push(Token::Subtract)
                },
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '%' => tokens.push(Token::Modulus),
                '^' => tokens.push(Token::Power),
                '=' => tokens.push(Token::Equal),
                '<' => {
                    let next_char = self.peek().unwrap();
                    if next_char == '=' {
                        tokens.push(Token::LessEqual);
                        self.next();
                    }
                    else if next_char == ' ' || next_char.is_alphanumeric() {
                        tokens.push(Token::Less);
                    }
                    else {
                        self.throw_error("Unexpected operator. Did you mean \"<=\"?");
                        return None;
                    }
                },
                '>' => {
                    let next_char = self.peek().unwrap();
                    if next_char == '=' {
                        tokens.push(Token::GreaterEqual);
                        self.next();
                    }
                    else if next_char == ' ' || next_char.is_alphanumeric() {
                        println!("{:?}", next_char);
                        tokens.push(Token::Greater);
                    }
                    else {
                        self.throw_error("Unexpected operator. Did you mean \">=\"?");
                        return None;
                    }
                },
                '.' => {
                    if self.peek().unwrap() == '.' {
                        tokens.push(Token::Range);
                        self.next();
                    }
                },
                '0'..='9' => self.handle_number(&mut tokens, cur_char, false),
                'i' => {
                    if self.peek().unwrap() == 'n' {
                        if self.peek_ahead(1).unwrap() == ' ' {
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

        Some(tokens)
    }
}