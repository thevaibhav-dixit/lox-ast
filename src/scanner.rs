use crate::error::LoxError;
use crate::object::Object;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        let res = match c {
            '(' => self.make_token(TokenType::LeftParen, None),
            ')' => self.make_token(TokenType::RightParen, None),
            '{' => self.make_token(TokenType::LeftBrace, None),
            '}' => self.make_token(TokenType::RightBrace, None),
            ',' => self.make_token(TokenType::Comma, None),
            '.' => self.make_token(TokenType::Dot, None),
            '-' => self.make_token(TokenType::Minus, None),
            '+' => self.make_token(TokenType::Plus, None),
            ';' => self.make_token(TokenType::Semicolon, None),
            '*' => self.make_token(TokenType::Star, None),
            // i am not adding the support for comments atm
            '/' => self.make_token(TokenType::Slash, None),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual, None)
                } else {
                    self.make_token(TokenType::Bang, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual, None)
                } else {
                    self.make_token(TokenType::Equal, None)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual, None)
                } else {
                    self.make_token(TokenType::Less, None)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual, None)
                } else {
                    self.make_token(TokenType::Greater, None)
                }
            }
            ' ' | '\r' | '\t' => return Ok(()),
            '\n' => {
                self.line += 1;
                return Ok(());
            }
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number()?,
            c if c.is_ascii() => self.identifier()?,

            _ => return Err(LoxError::UnexpectedCharacter(c)),
        };

        self.tokens.push(res);
        Ok(())
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current as usize]
    }

    fn identifier(&mut self) -> Result<Token, LoxError> {
        while self.peek().is_alphanumeric() {
            self.advance();
        }
        let text = self.source[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        let token_type = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        Ok(self.make_token(token_type, None))
    }

    fn string(&mut self) -> Result<Token, LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
        }
        if self.is_at_end() {
            return Err(LoxError::UnterminatedString);
        }
        self.advance();
        let value = self.source[(self.start + 1) as usize..(self.current - 1) as usize]
            .iter()
            .collect::<String>();
        Ok(self.make_token(TokenType::String, Some(Object::String(value))))
    }

    // this feels a wrong implementation to me
    // because it should ideally return an error for a case
    // like 12.3.43.5
    fn number(&mut self) -> Result<Token, LoxError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = self.source[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        Ok(self.make_token(
            TokenType::Number,
            Some(Object::Number(value.parse::<f64>()?)),
        ))
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            return '\0';
        }
        self.source[(self.current + 1) as usize]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current as usize] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn make_token(&mut self, token_type: TokenType, literal: Option<Object>) -> Token {
        let text = self.source[self.start as usize..self.current as usize]
            .iter()
            .collect::<String>();
        Token::new(token_type, text, literal, self.line)
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[(self.current - 1) as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scanner() {
        let source = "var a = 12".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        dbg!(tokens);
    }
}
