use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoxError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unterminated String")]
    UnterminatedString,
    #[error("Could not parse the number")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
