use std::ops::Range;

use crate::flag::Flag;

pub(crate) enum TokenKind {
    OBra,
    CBra,
    Comma,
    #[cfg(feature = "variable")]
    Colon,
    #[cfg(any(feature = "arithmetic_range", feature = "range_padding"))]
    Semicolon,
    #[cfg(feature = "range_padding")]
    Equal,
    #[cfg(feature = "arithmetic_range")]
    Plus,
    #[cfg(feature = "arithmetic_range")]
    Minus,
    #[cfg(feature = "char_range")]
    Char,
    Text,
    Number,
}

#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Token {
    kind: TokenKind,
    range: Range<usize>,
}

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum TokenizerError {
    NoData,
}

impl std::error::Error for TokenizerError {}
impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "Data is empty"),
        }
    }
}

pub(crate) struct Tokenizer<'a> {
    pub(crate) data: &'a str,
    pub(crate) flags: Flag,
}

impl<'a> Tokenizer<'a> {
    pub(crate) const fn new(data: &'a str, flags: Flag) -> Self {
        Self { data, flags }
    }

    pub(crate) fn tokenize(&self) -> Result<Vec<Token>, TokenizerError> {
        let data = self.data.to_string();
        if data.is_empty() {
            return Err(TokenizerError::NoData);
        }
        let mut vec = vec![];
        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn special_chars_outside_braces_should_not_be_a_problem() {
        todo!("Special chars outside curly braces should not throw Err")
    }
}
