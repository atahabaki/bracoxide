use crate::Range;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    OBra,
    CBra,
    Comma,
    #[cfg(feature = "variable")]
    Colon,
    #[cfg(any(
        feature = "numeric_range",
        feature = "char_range",
        feature = "emoji_range",
    ))]
    Range,
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

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    range: Range<usize>,
}

impl Token {
    pub const fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
    pub const fn from_start_end(kind: TokenKind, range_start: usize, range_end: usize) -> Self {
        Self {
            kind,
            range: Range {
                start: range_start,
                end: range_end,
            },
        }
    }
}

pub type Tokens = Vec<Token>;
