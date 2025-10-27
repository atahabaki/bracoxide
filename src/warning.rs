use crate::tokenizer::TokenizerWarning;

#[derive(Debug, PartialEq)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum Warning {
    Token(TokenizerWarning),
    // Parser,
}
