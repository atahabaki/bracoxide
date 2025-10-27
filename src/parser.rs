use crate::{
    Artifact,
    flag::Flag,
    tokenizer::{Token, Tokenizer, Tokens},
};

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum ParserError {
    NoToken,
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoToken => write!(f, "Tokens are empty."),
        }
    }
}

#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Parser<'a> {
    data: &'a str,
    flags: Flag,
    artifact: Artifact<Tokens>,
}

impl<'a> Parser<'a> {
    pub(crate) const fn from_tokenizer(
        tokenizer: Tokenizer<'a>,
        artifact: Artifact<Tokens>,
    ) -> Self {
        Parser {
            data: tokenizer.data,
            flags: tokenizer.flags,
            artifact,
        }
    }

    pub(crate) const fn parse(&self) -> Result<Vec<String>, ParserError> {
        if self.artifact.has_any_token() {
            return Err(ParserError::NoToken);
        }
        let mut possibilities = vec![];
        Ok(possibilities)
    }
}

#[cfg(test)]
mod test {}
