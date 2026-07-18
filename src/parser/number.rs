use crate::parser::{Parser, ParserError, ParserState};

pub fn number(parser: &Parser, state: &mut ParserState) -> Result<String, ParserError> {
    let mut chars = parser.content[state.get_byte_idx()..].chars();
    let mut parsed = String::new();
    let start_idx = state.get_idx();
    while let Some(ch) = chars.next() {
        match ch {
            _ if ch.is_numeric() => parsed.push(ch),
            _ => {
                // state.set_idx_tuple(start_id);
                // return Err(ParserError::ExpectedNumber);
            }
        }
        state.inc_idx_with_byte(ch.len_utf8());
    }
    if parsed.is_empty() {
        state.set_idx_tuple(start_idx);
        return Err(ParserError::ExpectedNumber);
    }
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_number() {
        let content = "12345";
        let parser = Parser::new(content);
        let mut state = ParserState::new();
        assert_eq!(number(&parser, &mut state), Ok("12345".to_string()));
    }

    #[test]
    fn test_with_emoji_included() {
        let content = "🫠 times 12";
        let parser = Parser::new(content);
        let mut state = ParserState::new();
        state.set_idx(11, 8);
        assert_eq!(number(&parser, &mut state), Ok("12".to_string()));
    }
}
