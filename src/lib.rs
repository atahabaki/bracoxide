use std::error::Error;

use crate::{flag::Flag, parser::Parser, tokenizer::Tokenizer};

pub(crate) mod flag;
pub(crate) mod parser;
pub(crate) mod tokenizer;

pub fn explode(data: &str) -> Result<Vec<String>, Box<dyn Error>> {
    explosion(data, Flag::default())
}

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
