use std::error::Error;

use crate::{flag::Flag, parser::Parser, tokenizer::Tokenizer};

pub(crate) mod flag;
pub(crate) mod parser;
pub(crate) mod tokenizer;

pub fn explode<'a>(data: &'a str) -> Result<Vec<String>, Box<dyn Error>> {
    _explode(data, Flag::default())
}

pub fn explode_with_flags<'a>(data: &'a str, flags: Flag) -> Result<Vec<String>, Box<dyn Error>> {
    _explode(data, flags)
}

fn _explode<'a>(data: &'a str, flags: Flag) -> Result<Vec<String>, Box<dyn Error>> {
    let tokenizer = Tokenizer::new(data, flags);
    let tokens = tokenizer.tokenize()?;
    let parser = Parser::from_tokenizer(tokenizer, tokens);
    let possibilities = parser.parse()?;
    Ok(possibilities)
}

#[cfg(test)]
mod tests {
    use crate::explode;

    #[test]
    fn simple_a_b() {
        assert_eq!(
            explode("{A,B}").unwrap(),
            vec!["A".to_string(), "B".to_string()]
        )
    }

    #[test]
    fn simple_a_b_c_d_e_f_g() {
        assert_eq!(
            explode("{A,B,C,D,E,F,G}").unwrap(),
            vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
                "F".to_string(),
                "G".to_string()
            ]
        )
    }

    #[test]
    fn simple_range() {
        assert_eq!(
            explode("{1..3}").unwrap(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        )
    }

    #[test]
    #[cfg(feature = "range_padding")]
    fn range_padding_zero_90_to_100() {
        assert_eq!(
            explode("{90..100;0=}").unwrap(),
            vec![
                "090".to_string(),
                "091".to_string(),
                "092".to_string(),
                "093".to_string(),
                "094".to_string(),
                "095".to_string(),
                "096".to_string(),
                "097".to_string(),
                "098".to_string(),
                "099".to_string(),
                "100".to_string(),
            ]
        )
    }

    #[test]
    #[cfg(feature = "range_padding")]
    fn range_padding_zero_9_to_10() {
        assert_eq!(
            explode("{7..10;100_00=}").unwrap(),
            vec![
                "100_007".to_string(),
                "100_008".to_string(),
                "100_009".to_string(),
                "100_010".to_string(),
            ]
        )
    }

    #[test]
    #[cfg(feature = "range_padding")]
    fn range_padding_zero_10_to_7() {
        assert_eq!(
            explode("{10..7;=00_000}").unwrap(),
            vec![
                "700_000".to_string(),
                "800_000".to_string(),
                "900_000".to_string(),
                "100_000".to_string(),
            ]
        )
    }
}
