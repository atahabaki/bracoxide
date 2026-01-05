use crate::{parser::{error::ParserError, state::ParserState}};

pub fn number<'a>(input: &'a str, _state: &mut ParserState) -> Result<(&'a str, String), ParserError> {
    let mut chars = input.chars();
    let mut parsed = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            _ if ch.is_numeric() => parsed.push(ch),
            _ => return Err(ParserError::ExpectedNumber),
        }
    }
    let next_index = parsed.len();
    Ok((&input[next_index..], parsed))
}

#[cfg(test)]
mod tests {
    use crate::parser::state::ParserState;

    use super::*;

    #[test]
    fn test_basic_number() {
        let content = "12345";
        let mut state = ParserState::new(content);
        assert_eq!(
            Ok(("", "12345".to_string())),
            number(content, &mut state)
        )
    }
}