use crate::{flag::Flag, parser::Parser, tokenizer::Tokenizer};

pub(crate) mod artifact;
pub(crate) mod flag;
pub(crate) mod parser;
pub(crate) mod phase;
pub(crate) mod tokenizer;
pub(crate) mod warning;

pub(crate) use artifact::Artifact;
pub(crate) use core::error::Error;
pub(crate) use core::fmt::{Display, Formatter, Result as FmtResult};
pub(crate) use core::iter::{Enumerate, Peekable};
pub(crate) use core::ops::Range;
pub(crate) use core::result::Result;
pub(crate) use core::str::Chars;
pub(crate) use phase::Phase;
pub(crate) use warning::Warning;

/// # Errors
///
/// - Empty content/data/String
/// - Somehow tokenizer gets No Token
// TODO: Update whenever new Error type gets
pub fn explode(data: &str) -> Result<Vec<String>, Box<dyn Error>> {
    explosion(data, Flag::default())
}

/// # Errors
///
/// - Empty content/data/String
/// - Somehow tokenizer gets No Token
// TODO: Update whenever new Error type gets
pub fn explode_with_flags(data: &str, flags: Flag) -> Result<Vec<String>, Box<dyn Error>> {
    explosion(data, flags)
}

fn explosion(data: &str, flags: Flag) -> Result<Vec<String>, Box<dyn Error>> {
    let tokenizer = Tokenizer::new(data, flags);
    let tokens = tokenizer.tokenize()?;
    let parser = Parser::from_tokenizer(tokenizer, tokens);
    let possibilities = parser.parse()?;
    Ok(possibilities)
}
