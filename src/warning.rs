use crate::tokenizer::TokenizerWarning;

#[derive(Debug, PartialEq, Eq)]
pub enum Warning {
    Token(TokenizerWarning),
    // Parser,
}
