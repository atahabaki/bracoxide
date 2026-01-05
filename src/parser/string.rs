use crate::parser::{error::ParserError, state::ParserState};

pub fn string_inside_braces<'a>(
    input: &'a str,
    state: &mut ParserState,
) -> Result<(&'a str, String), ParserError> {
    let escape_char = state.get_escape_char();
    let mut chars = input.chars();
    let mut is_escape = (false, 0);
    let mut parsed = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            _ if is_escape.0 => {
                parsed.push(ch);
                is_escape.0 = false;
            }
            _ if ch == escape_char => {
                is_escape.0 = true;
                is_escape.1 += 1;
                continue;
            }
            #[cfg(feature = "variable")]
            ':' => {
                return Err(ParserError::ExpectedString);
            }
            #[cfg(feature = "arithmetic_range")]
            '+' | '-' => return Err(ParserError::ExpectedString),
            #[cfg(any(feature = "range_padding", feature = "arithmetic_range"))]
            ';' | '=' => return Err(ParserError::ExpectedString),
            #[cfg(any(feature = "numeric_range", feature = "unicode_range"))]
            '.' => return Err(ParserError::ExpectedString),
            '{' | '}' | ',' | _ if ch.is_numeric() => {
                return Err(ParserError::ExpectedString);
            }
            _ => {
                parsed.push(ch);
            }
        }
    }
    let position = parsed.len() + is_escape.1;
    state.set_position(position);
    Ok((&input[position..], parsed))
}

pub fn string_outside_braces<'a>(
    input: &'a str,
    state: &mut ParserState,
) -> Result<(&'a str, String), ParserError> {
    let escape_char = state.get_escape_char();
    let mut chars = input.chars();
    let mut is_escape = (false, 0);
    let mut parsed = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            _ if is_escape.0 => {
                parsed.push(ch);
                is_escape.0 = false;
            }
            _ if ch == escape_char => {
                is_escape.0 = true;
                is_escape.1 += 1;
                continue;
            }
            '{' | '}' | _ if ch.is_numeric() => {
                return Err(ParserError::ExpectedString);
            }
            _ => {
                parsed.push(ch);
            }
        }
    }
    let position = parsed.len() + is_escape.1;
    state.set_position(position);
    Ok((&input[position..], parsed))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_basic_string() {
        let content = "Hi, this is a string!";
        let mut state = ParserState::new(content);
        assert_eq!(
            Ok(("", content.to_string())),
            string_outside_braces(content, &mut state),
        );
    }

    #[test]
    fn test_escaped_string() {
        let content = "Hi, this one has %{ escape sequence%:";
        let mut state = ParserState::new(content);
        assert_eq!(
            Ok(("", "Hi, this one has { escape sequence:".to_string())),
            string_outside_braces(content, &mut state)
        );
    }
}
