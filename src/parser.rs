use crate::{
    flag::Flag,
    tokenizer::{Token, Tokenizer},
};

#[derive(Debug)]
pub(crate) enum ParserError {
    NoToken,
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NoToken => write!(f, "Tokens are empty."),
        }
    }
}

pub(crate) struct Parser<'a> {
    data: &'a str,
    flags: Flag,
    tokens: Vec<Token>,
}

impl<'a> Parser<'a> {
    pub(crate) fn from_tokenizer(tokenizer: Tokenizer<'a>, tokens: Vec<Token>) -> Self {
        Parser {
            data: tokenizer.data,
            flags: tokenizer.flags,
            tokens,
        }
    }

    pub(crate) fn parse(&self) -> Result<Vec<String>, ParserError> {
        if self.tokens.is_empty() {
            return Err(ParserError::NoToken);
        }
        let mut possibilities = vec![];
        Ok(possibilities)
    }
}

#[cfg(test)]
mod test {}
