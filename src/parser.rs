use crate::lexer::Token;

pub struct Parser {
    current: usize,
    src: Vec<Token>
}

impl Parser {
    pub fn new(src: Vec<Token>) -> Parser {
        Parser{current: 0, src: src}
    }

    fn next(&mut self) -> &Token {
        let token = &self.src[self.current];
        self.current += 1;
        token
    }

    fn peek(&mut self) -> &Token {
        &self.src[self.current]
    }

    fn peek_ahead(&mut self, offset: usize) -> &Token {
        &self.src[self.current + offset]
    }

    // TOOD: what does this actually return?
    pub fn parse(&mut self) {
        while let cur_tok = self.next() {
            match cur_tok {
                Token::LeftBrace => {},
                Token::RightBrace => {},
                Token::LeftBracket => {},
                Token::RightBracket => {},
                Token::Pipe => {},
                Token::Add => {},
                Token::Subtract => {},
                Token::Multiply => {},
                Token::Divide => {},
                Token::Modulus => {},
                Token::Power => {},
                Token::Equal => {},
                Token::Less => {},
                Token::LessEqual => {},
                Token::Greater => {},
                Token::GreaterEqual => {},
                Token::Range => {},
                Token::Number(number) => {},
                // TODO: handle not finding a '|' here?
                Token::In => {},
                Token::Identifier(identifier) => {},
                Token::End => break,
                _ => {}
            }
        }
    }
}