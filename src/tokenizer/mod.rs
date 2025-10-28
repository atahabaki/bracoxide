pub mod error;
pub mod state;
pub mod token;
pub mod warning;

use crate::{Artifact, Warning, flag::Flag};
use error::TokenizerError;
use state::{BufferState, TokenizerState};
pub(crate) use token::{Token, TokenKind, Tokens};
pub(crate) use warning::{TokenizerWarning, TokenizerWarnings};

#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Tokenizer<'a> {
    pub(crate) data: &'a str,
    pub(crate) flags: Flag,
}

impl<'a> Tokenizer<'a> {
    pub(crate) const fn new(data: &'a str, flags: Flag) -> Self {
        Self { data, flags }
    }

    pub(crate) fn tokenize(&self) -> Result<Artifact<Tokens>, TokenizerError> {
        let data = self.data.to_string();
        if data.is_empty() {
            return Err(TokenizerError::NoData);
        }
        let mut tokens = vec![];
        let mut warnings = vec![];
        let mut iter = data.chars().enumerate();
        let escape_char = self.flags.escape_char;
        let suppress_warning = self.flags.supress_warning;
        let mut state = TokenizerState::default();
        while let Some((i, c)) = iter.next() {
            #[cfg(debug_assertions)]
            {
                println!("{:?}", state);
            }
            match c {
                // Previously tokenizer met with actual escape char
                _ if state.is_escape() => {
                    // I can not see any particular reason to use escape char to
                    // escape not special characters so, im making a choice here
                    // escape char is only meant to escape special chars treated as text.
                    // and some special chars are only meaningful in curly brackets, so
                    // if they're not in curly brackets they're also treated as text
                    // so escaping them outside braces are gonna throw warnings around.
                    state.new_range(i);
                    match c {
                        _ if c == escape_char => {
                            state.set_state_text();
                        }
                        '{' | ',' | '}' => {
                            state.set_state_text();
                        }
                        #[cfg(any(
                            feature = "numeric_range",
                            feature = "char_range",
                            feature = "emoji_range"
                        ))]
                        '.' if state.is_inside_curly_brackets() => {
                            state.set_state_text();
                        }
                        #[cfg(any(feature = "range_padding", feature = "arithmetic_range"))]
                        '=' if state.is_inside_curly_brackets() => {
                            state.set_state_text();
                        }
                        #[cfg(feature = "variable")]
                        ':' if state.is_inside_curly_brackets() => {
                            state.set_state_text();
                        }
                        #[cfg(any(feature = "range_padding", feature = "arithmetic_range"))]
                        ';' if state.is_inside_curly_brackets() => {
                            state.set_state_text();
                        }
                        #[cfg(feature = "arithmetic_range")]
                        '+' | '-' if state.is_inside_curly_brackets() => {
                            state.set_state_text();
                        }
                        _ => {
                            state.set_state_text();
                            if !suppress_warning {
                                warnings.push(Warning::Token(TokenizerWarning::RedundantEscape {
                                    position: i,
                                }));
                            }
                        }
                    }
                    state.set_escape(false);
                }
                // tokenizer met with actual escape char
                _ if c == escape_char => {
                    // 1. push token based on the previous buffer state
                    match state.get_previous_buffer_state() {
                        Some(BufferState::Escape) => unreachable!(),
                        Some(BufferState::Text) => {
                            let token = Token::new(TokenKind::Text, state.get_range());
                            tokens.push(token);
                        }
                        Some(BufferState::Number) => {
                            let token = Token::new(TokenKind::Number, state.get_range());
                            tokens.push(token);
                        }
                        // it is sth. like
                        // {a,%
                        //    ^
                        // What would you do, start a new range?
                        Some(BufferState::TokenPushed) => (),
                        None => (),
                    }
                    // 2. continue
                    state.set_escape(true);
                }
                '{' => {
                    state.increment_obra();
                }
                '}' => {
                    state.increment_cbra();
                }
                ',' if state.is_inside_curly_brackets() => {}
                #[cfg(any(
                    feature = "numeric_range",
                    feature = "char_range",
                    feature = "emoji_range"
                ))]
                '.' if state.is_inside_curly_brackets() => {}
                #[cfg(any(feature = "range_padding", feature = "arithmetic_range"))]
                '=' if state.is_inside_curly_brackets() => {}
                #[cfg(feature = "variable")]
                ':' if state.is_inside_curly_brackets() => {}
                #[cfg(any(feature = "range_padding", feature = "arithmetic_range"))]
                ';' if state.is_inside_curly_brackets() => {}
                #[cfg(feature = "arithmetic_range")]
                '+' | '-' if state.is_inside_curly_brackets() => {}
                // NOTE: This one also get non 0-9 numerical digits, arabic etc. They're included to
                // Not sure, this is what i want, but we'll see...
                _ if c.is_numeric() => match state.get_previous_buffer_state() {
                    // we dealt with this possibility above if somehow reaches to this arm, then pls. for god's sake throw fucking error.
                    Some(BufferState::Escape) => unreachable!(),
                    Some(BufferState::Text) => state.increment_range_end(),
                    Some(BufferState::Number) => state.increment_range_end(),
                    Some(BufferState::TokenPushed) => {
                        state.new_range(i);
                        state.set_state_number();
                        state.increment_range_end();
                    }
                    None => {
                        state.set_state_number();
                        state.increment_range_end();
                    }
                },
                _ => match state.get_previous_buffer_state() {
                    Some(BufferState::Escape) => unreachable!(),
                    Some(BufferState::Text) => state.increment_range_end(),
                    Some(BufferState::Number) => {
                        state.set_state_text();
                        state.increment_range_end();
                    }
                    Some(BufferState::TokenPushed) => {
                        state.new_range(i);
                        state.set_state_text();
                    }
                    None => {
                        state.set_state_text();
                        state.increment_range_end();
                    }
                },
            }
        }
        #[cfg(debug_assertions)]
        {
            println!("{:?}", state);
        }
        match state.get_previous_buffer_state() {
            // This arm is kinda like 'banana1345%' where the % is the escape, how should we handle this?
            // let me think, or add a flag for it
            Some(BufferState::Escape) => todo!(),
            Some(BufferState::Text) => {
                let token = Token::new(TokenKind::Text, state.get_range());
                tokens.push(token);
            }
            Some(BufferState::Number) => {
                let token = Token::new(TokenKind::Number, state.get_range());
                tokens.push(token);
            }
            Some(BufferState::TokenPushed) => {
                if tokens.len() > 0 {
                    ()
                }
            }
            None => (),
        }
        Ok(Artifact::with_warnings(tokens, warnings))
    }
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use super::*;

    #[test]
    fn tokenizer_state_inc_dec() {
        let mut state = TokenizerState::default();
        state.increment_range_end();
        let expected_range = Range { start: 0, end: 1 };
        assert_eq!(state.get_range(), expected_range);
        state.decrement_range_end();
        let expected_range = Range { start: 0, end: 0 };
        assert_eq!(state.get_range(), expected_range);
    }

    fn the_rest(content: &str, expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(content, Flag::default());
        let tokens = tokenizer.tokenize();
        assert!(tokens.is_ok());
        let artifact = Artifact::<Tokens>::new(expected_tokens);
        assert_eq!(tokens.unwrap(), artifact);
    }
    #[test]
    fn double_escape_considered_as_text() {
        let content = "%%";
        let expected_tokens = vec![Token::from_start_end(TokenKind::Text, 1, 2)];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_before_after_escape() {
        let content = "E..s %{apple,banana%}...";
        let expected_tokens = vec![
            Token::new(TokenKind::Text, Range { start: 0, end: 5 }), // 'E..s '
            Token::from_start_end(TokenKind::Text, 6, 19),           // '{apple,banana'
            Token::from_start_end(TokenKind::Text, 20, 24),          // '}...'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_number_in_braces() {
        let content = "{banana,1345}";
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::OBra, 0, 1),
            Token::from_start_end(TokenKind::Text, 1, 7),
            Token::from_start_end(TokenKind::Comma, 7, 8),
            Token::from_start_end(TokenKind::Number, 8, 12),
            Token::from_start_end(TokenKind::CBra, 12, 13),
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_number() {
        let content = "banana1345";
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::Text, 0, 10), // 'banana1345'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_interrupted_by_escape() {
        let content = "banana%%1345";
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::Text, 0, 6), // 'banana'
            Token::from_start_end(TokenKind::Text, 7, 12), // '%1345'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn pure_number() {
        let content = "1345";
        let expected_tokens = vec![Token::from_start_end(TokenKind::Number, 0, 4)];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn number_text() {
        let content = "1345banana";
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::Text, 0, 10), // '1345banana'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn special_chars_outside_braces_should_not_be_a_problem() {
        todo!("Special chars outside curly braces should not throw Err")
    }
}
