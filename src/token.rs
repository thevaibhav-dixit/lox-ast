use crate::object::Object;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Object>, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

// impl std::fmt::Display for Token {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let literal = match &self.literal {
//             Some(l) => l,
//             None => "",
//         };
//         write!(f, "{:?} {} {}", self.token_type, self.lexeme, literal)
//     }
// }
