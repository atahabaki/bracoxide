use crate::{
    Artifact,
    flag::Flag,
    tokenizer::{Tokenizer, TokenizerWarning, Tokens},
};

#[derive(Debug)]
pub enum ParserError {
    NoToken,
}

impl crate::Error for ParserError {}
impl crate::Display for ParserError {
    fn fmt(&self, f: &mut crate::Formatter<'_>) -> crate::FmtResult {
        match self {
            Self::NoToken => write!(f, "Tokens are empty."),
        }
    }
}

pub struct Parser<'a> {
    data: &'a str,
    flags: Flag,
    artifact: Artifact<Tokens, TokenizerWarning>,
}

impl<'a> Parser<'a> {
    pub const fn from_tokenizer(
        tokenizer: Tokenizer<'a>,
        artifact: Artifact<Tokens, TokenizerWarning>,
    ) -> Self {
        Parser {
            data: tokenizer.data,
            flags: tokenizer.flags,
            artifact,
        }
    }

    pub const fn parse(&self) -> Result<Vec<String>, ParserError> {
        if self.artifact.has_any_token() {
            return Err(ParserError::NoToken);
        }
        let possibilities = vec![];
        Ok(possibilities)
    }
}

#[cfg(test)]
mod test {}
