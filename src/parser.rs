use crate::{lexer::Token, setbuilder::{SetBuilder, Operation, ElementOperation, ElementFilter, Comparator}};

pub struct Parser {
    current: usize,
    src: Vec<Token>
}

impl Parser {
    pub fn new(src: Vec<Token>) -> Parser {
        Parser{current: 0, src: src}
    }

    fn next(&mut self) -> Option<&Token> {
        let token = &self.src[self.current];
        if self.current + 1 >= self.src.len() {
            None
        }
        else {
            self.current += 1;
            Some(token)
        }
    }

    fn peek(&self) -> &Token {
        &self.src[self.current]
    }

    fn peek_ahead(&self, offset: usize) -> Option<&Token> {
        if self.current + offset >= self.src.len() {
            None
        }
        else {
            Some(&self.src[self.current + offset])
        }
    }
}

// Why isn't this a method of Parser?
// Because if we pass a '&mut self' to this
// method, then the borrow checker gets upset
// that I'm taking multiple mutable references
// to 'self' when I call the `peek` method
// inside the `while let Some(cur_tok) = self.next()`
// block. I don't have any better ideas about how to
// get around that.
pub fn parse(parser: &mut Parser) -> SetBuilder {
    let mut element_operation = ElementOperation::new();
    let mut element_filter = ElementFilter::new();
    let mut range = std::ops::Range { start: 0, end: 0 };
    let mut in_predicate = false;
    let mut in_range = false;
    while let Some(cur_tok) = parser.next() {
        match cur_tok {
            &Token::LeftBrace => continue,
            &Token::RightBrace => continue,
            &Token::LeftBracket => {
                if !in_range {
                    todo!("Error handling for range not found after \"in\".")
                }
            },
            &Token::RightBracket => in_range = false,
            &Token::In => in_range = true,
            &Token::Range => {
                if in_range {
                    continue
                }
            },
            &Token::Pipe => in_predicate = true,
            &Token::Add => {
                if !in_predicate {
                    element_operation.operation = Operation::Add;
                }
                else {
                    element_filter.operation = Operation::Add;
                }
            },
            &Token::Subtract => {
                if !in_predicate {
                    element_operation.operation = Operation::Subtract;
                }
                else {
                    element_filter.operation = Operation::Subtract;
                }
            },
            &Token::Multiply => {
                if !in_predicate {
                    element_operation.operation = Operation::Multiply;
                }
                else {
                    element_filter.operation = Operation::Multiply;
                }
            },
            &Token::Divide => {
                if !in_predicate {
                    element_operation.operation = Operation::Divide;
                }
                else {
                    element_filter.operation = Operation::Divide;
                }
            },
            &Token::Modulus => {
                if !in_predicate {
                    element_operation.operation = Operation::Modulus;
                }
                else {
                    element_filter.operation = Operation::Modulus;
                }
            },
            &Token::Power => {
                if !in_predicate {
                    element_operation.operation = Operation::Power;
                }
                else {
                    element_filter.operation = Operation::Power;
                }
            },
            &Token::Equal => element_filter.comparison = Comparator::Equal,
            &Token::NotEqual => element_filter.comparison = Comparator::NotEqual,
            &Token::Less => element_filter.comparison = Comparator::Less,
            &Token::LessEqual => element_filter.comparison = Comparator::LessEqual,
            &Token::Greater => element_filter.comparison = Comparator::Greater,
            &Token::GreaterEqual => element_filter.comparison = Comparator::GreaterEqual,
            &Token::Number(number) => {
                if !in_predicate {
                    element_operation.operand = number;
                }
                else if in_range {
                    if parser.peek() == &Token::RightBracket {
                        range.end = number as i64;
                    }
                    else if parser.peek() == &Token::Range {
                        range.start = number as i64;
                    }
                    else {
                        todo!("Parser range error handling")
                    }
                }
                else if element_filter.comparison != Comparator::None {
                    element_filter.operand = number;
                }
                else {
                    element_filter.compare_to = number;
                }
            },
            Token::Identifier(identifier) => {
                if !in_predicate {
                    // Calling '.to_string()' on a string... Rust moment
                    element_operation.identifier = identifier.to_string();
                }
                else {
                    element_filter.identifier = identifier.to_string();
                }
            },
            Token::End => break
        }
    }
    SetBuilder { range, element_operation, element_filter }
}