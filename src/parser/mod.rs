pub mod error;
pub mod literal;
pub mod number;
pub mod state;
pub mod string;

pub use error::ParserError;
pub use state::ParserState;

use crate::Flag;

pub struct Parser<'a> {
    content: &'a str,
    escape_char: char,
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Self::with_flag(content, Flag::default())
    }
    pub fn with_flag(content: &'a str, flag: Flag) -> Self {
        Self {
            content,
            escape_char: flag.escape_char,
        }
    }
}
