use std::ops::Range;

#[derive(Debug, PartialEq)]
#[allow(clippy::redundant_pub_crate)]
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

#[derive(Debug, PartialEq)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Token {
    kind: TokenKind,
    range: Range<usize>,
}

impl Token {
    pub(crate) const fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
    pub(crate) const fn from_start_end(
        kind: TokenKind,
        range_start: usize,
        range_end: usize,
    ) -> Self {
        Self {
            kind,
            range: Range {
                start: range_start,
                end: range_end,
            },
        }
    }
}

pub(crate) type Tokens = Vec<Token>;
