use crate::parser::{Parser, ParserError, ParserState};

/// Returns a function which returns if the expected and the content matches 100%
/// if not returns the expected in Err.
///
/// Useful for building operators such as:
/// ```rust,ignore,no_run,hidden
/// let obra = literal("{");
/// let cbra = literal("}");
/// let range_opr = literal("..");
/// let comma = literal(",");
/// let semicolon = literal(";");
/// let colon = literal(":");
/// let eq = literal("=");
/// let plus = literal("+");
/// let minus = literal("-");
/// ```
pub fn literal<'a>(
    expected: &'a str,
) -> impl Fn(&Parser<'a>, &mut ParserState) -> Result<String, ParserError> {
    move |parser, state| -> Result<String, ParserError> {
        let start_idx = state.get_idx();
        let expected_len = expected.len();
        let end_idx = expected_len + start_idx.0;
        return match parser.content.get(start_idx.0..end_idx) {
            Some(next) if next == expected => {
                println!("{next}");
                println!("{expected}");
                state.inc_idx_by(expected_len, expected.chars().count());
                Ok(expected.to_string())
            }
            _ => Err(ParserError::Expected(expected.to_string())),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_literal() {
        let parse_obra = literal("{");
        assert_eq!(
            Ok("{".to_string()),
            parse_obra(&Parser::new("{"), &mut ParserState::new())
        );
        let parse_cbra = literal("}");
        assert_eq!(
            Ok("}".to_string()),
            parse_cbra(&Parser::new("}"), &mut ParserState::new())
        );
        let range_opr = literal("..");
        assert_eq!(
            Ok("..".to_string()),
            range_opr(&Parser::new(".."), &mut ParserState::new())
        );
        let just4test = literal("..{}");
        assert_eq!(
            Err(ParserError::Expected("..{}".to_string())),
            just4test(&Parser::new("..{"), &mut ParserState::new())
        );
    }
}
