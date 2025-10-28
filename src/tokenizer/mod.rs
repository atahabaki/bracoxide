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
        let mut iter = data.chars().enumerate().peekable();
        let escape_char = self.flags.escape_char;
        let suppress_warning = self.flags.supress_warning;
        let mut state = TokenizerState::default();
        while let Some((i, c)) = iter.next() {
            #[cfg(debug_assertions)]
            {
                println!("{state:?}");
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
                        // None | Some(BufferState::TokenPushed) => (),
                        _ => (),
                    }
                    // 2. continue
                    state.set_escape(true);
                }
                '{' => {
                    match state.get_previous_buffer_state() {
                        Some(BufferState::Escape) => unreachable!(),
                        Some(buf_state) => {
                            #[cfg(debug_assertions)]
                            {
                                println!("{:?}, {i:?}", state.get_range());
                            }
                            let token = Token::new(
                                match buf_state {
                                    BufferState::Escape | BufferState::TokenPushed => {
                                        unreachable!()
                                    }
                                    BufferState::Text => TokenKind::Text,
                                    BufferState::Number => TokenKind::Number,
                                },
                                state.get_range(),
                            );
                            tokens.push(token);
                            state.set_state_token();
                        }
                        // None | Some(BufferState::TokenPushed) => (),
                        _ => (),
                    }
                    state.increment_obra();
                    state.new_range(i);
                    let token = Token::new(TokenKind::OBra, state.get_range());
                    tokens.push(token);
                    state.set_state_token();
                }
                '}' if state.is_inside_curly_brackets() => {
                    match state.get_previous_buffer_state() {
                        Some(BufferState::TokenPushed) => (),
                        Some(buf_state) => {
                            state.set_range_end_if_biggers_than(i);
                            let token = Token::new(
                                match buf_state {
                                    BufferState::Text => TokenKind::Text,
                                    BufferState::Number => TokenKind::Number,
                                    _ => unreachable!(),
                                },
                                state.get_range(),
                            );
                            tokens.push(token);
                        }
                        // None | Some(BufferState::Escape) => unreachable!(),
                        _ => unreachable!(),
                    }
                    state.new_range(i);
                    let token = Token::new(TokenKind::CBra, state.get_range());
                    tokens.push(token);
                    state.increment_cbra();
                    state.set_state_token();
                }
                ',' if state.is_inside_curly_brackets() => {
                    match state.get_previous_buffer_state() {
                        Some(BufferState::TokenPushed) => (),
                        Some(buf_state) => {
                            let token = Token::new(
                                match buf_state {
                                    BufferState::Number => TokenKind::Number,
                                    BufferState::Text => TokenKind::Text,
                                    _ => unreachable!(),
                                },
                                state.get_range(),
                            );
                            tokens.push(token);
                            state.set_state_token();
                        }
                        // None | Some(BufferState::Escape) => unreachable!(),
                        _ => (),
                    }
                    let token = Token::from_start_end(TokenKind::Comma, i, i + 1);
                    tokens.push(token);
                    state.set_state_token();
                }
                #[cfg(any(
                    feature = "numeric_range",
                    feature = "char_range",
                    feature = "emoji_range"
                ))]
                '.' if state.is_inside_curly_brackets() => {
                    // it is already in {}
                    match state.get_previous_buffer_state() {
                        None | Some(BufferState::Escape) => unreachable!(),
                        Some(buf_state) => match iter.peek() {
                            Some((ix, cx)) => match cx {
                                '.' => {
                                    match buf_state {
                                        BufferState::Escape => unreachable!(),
                                        BufferState::Text => {
                                            state.set_range_end_if_biggers_than(i);
                                            let token =
                                                Token::new(TokenKind::Text, state.get_range());
                                            tokens.push(token);
                                            state.set_state_token();
                                        }
                                        BufferState::Number => {
                                            state.set_range_end_if_biggers_than(i);
                                            let token =
                                                Token::new(TokenKind::Number, state.get_range());
                                            tokens.push(token);
                                            state.set_state_token();
                                        }
                                        BufferState::TokenPushed => (),
                                    }
                                    state.new_range(i);
                                    state.increment_range_end();
                                    let token = Token::new(TokenKind::Range, state.get_range());
                                    tokens.push(token);
                                    state.set_state_token();
                                    iter.next();
                                }
                                _ => {
                                    state.set_state_text();
                                }
                            },
                            None => match state.get_previous_buffer_state() {
                                None | Some(BufferState::Escape) => unreachable!(),
                                Some(BufferState::TokenPushed) => {
                                    state.new_range(i);
                                    state.set_state_text();
                                }
                                Some(_) => {
                                    state.set_state_text();
                                    state.increment_range_end();
                                }
                            },
                        },
                    }
                }
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
                    Some(BufferState::TokenPushed) => {
                        state.new_range(i);
                        state.set_state_number();
                        state.increment_range_end();
                    }
                    Some(_) => state.increment_range_end(),
                    None => {
                        state.set_state_number();
                        state.increment_range_end();
                    }
                },
                _ => match state.get_previous_buffer_state() {
                    Some(BufferState::Escape) => unreachable!(),
                    Some(BufferState::Text) => state.increment_range_end(),
                    None | Some(BufferState::Number) => {
                        state.set_state_text();
                        state.increment_range_end();
                    }
                    Some(BufferState::TokenPushed) => {
                        state.new_range(i);
                        state.set_state_text();
                    }
                },
            }
        }
        #[cfg(debug_assertions)]
        {
            println!("{state:?}");
        }
        match state.get_previous_buffer_state() {
            // This arm is kinda like 'banana1345%' where the % is the escape, how should we handle this?
            // let me think, or add a flag for it
            Some(BufferState::Escape) => todo!(),
            Some(BufferState::Text) => {
                state.set_range_end_if_biggers_than(self.data.len());
                let token = Token::new(TokenKind::Text, state.get_range());
                tokens.push(token);
            }
            Some(BufferState::Number) => {
                state.set_range_end_if_biggers_than(self.data.len());
                let token = Token::new(TokenKind::Number, state.get_range());
                tokens.push(token);
            }
            None | Some(BufferState::TokenPushed) => (),
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
        assert_eq!("%", &content[1..2]);
        let expected_tokens = vec![Token::from_start_end(TokenKind::Text, 1, 2)];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_before_after_escape() {
        let content = "E..s %{apple,banana%}...";
        assert_eq!("E..s ", &content[0..5]);
        assert_eq!("{apple,banana", &content[6..19]);
        assert_eq!("}...", &content[20..24]);
        let expected_tokens = vec![
            Token::new(TokenKind::Text, Range { start: 0, end: 5 }),
            Token::from_start_end(TokenKind::Text, 6, 19),
            Token::from_start_end(TokenKind::Text, 20, 24),
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_number_in_braces() {
        let content = "{banana,1345}";
        assert_eq!("{", &content[0..1]);
        assert_eq!("banana", &content[1..7]);
        assert_eq!(",", &content[7..8]);
        assert_eq!("1345", &content[8..12]);
        assert_eq!("}", &content[12..13]);
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
        assert_eq!("banana1345", &content[0..10]);
        let expected_tokens = vec![Token::from_start_end(TokenKind::Text, 0, 10)];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn text_interrupted_by_escape() {
        let content = "banana%%1345";
        assert_eq!("banana", &content[0..6]);
        assert_eq!("%1345", &content[7..12]);
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::Text, 0, 6),
            Token::from_start_end(TokenKind::Text, 7, 12),
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn pure_number() {
        let content = "1345";
        assert_eq!("1345", &content[0..4]);
        let expected_tokens = vec![Token::from_start_end(TokenKind::Number, 0, 4)];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn number_text() {
        let content = "1345banana";
        assert_eq!("1345banana", &content[0..10]);
        let expected_tokens = vec![Token::from_start_end(TokenKind::Text, 0, 10)];
        the_rest(content, expected_tokens);
    }
    #[cfg(any(
        feature = "numeric_range",
        feature = "char_range",
        feature = "emoji_range"
    ))]
    mod dot {
        use super::*;

        #[test]
        fn last_one_is_dot_after_text_in_brace() {
            let content = "{a,b.";
            assert_eq!("{", &content[0..1]);
            assert_eq!("a", &content[1..2]);
            assert_eq!(",", &content[2..3]);
            assert_eq!("b.", &content[3..5]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::OBra, 0, 1),
                Token::from_start_end(TokenKind::Text, 1, 2),
                Token::from_start_end(TokenKind::Comma, 2, 3),
                Token::from_start_end(TokenKind::Text, 3, 5),
            ];
            the_rest(content, expected_tokens);
        }
        #[test]
        fn last_one_is_dot_in_brace() {
            let content = "{a,.";
            assert_eq!("{", &content[0..1]);
            assert_eq!("a", &content[1..2]);
            assert_eq!(",", &content[2..3]);
            assert_eq!(".", &content[3..4]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::OBra, 0, 1),
                Token::from_start_end(TokenKind::Text, 1, 2),
                Token::from_start_end(TokenKind::Comma, 2, 3),
                Token::from_start_end(TokenKind::Text, 3, 4),
            ];
            the_rest(content, expected_tokens);
        }

        #[test]
        #[cfg(feature = "numeric_range")]
        fn numeric_range_without_start() {
            let content = "{..10";
            assert_eq!("{", &content[0..1]);
            assert_eq!("..", &content[1..3]);
            assert_eq!("10", &content[3..5]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::OBra, 0, 1),
                Token::from_start_end(TokenKind::Range, 1, 3),
                Token::from_start_end(TokenKind::Number, 3, 5),
            ];
            the_rest(content, expected_tokens);
        }

        #[test]
        #[cfg(feature = "numeric_range")]
        fn numeric_range_without_end() {
            let content = "{10..";
            assert_eq!("{", &content[0..1]);
            assert_eq!("10", &content[1..3]);
            assert_eq!("..", &content[3..5]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::OBra, 0, 1),
                Token::from_start_end(TokenKind::Number, 1, 3),
                Token::from_start_end(TokenKind::Range, 3, 5),
            ];
            the_rest(content, expected_tokens);
        }

        #[test]
        #[cfg(feature = "numeric_range")]
        fn numeric_range() {
            let content = "{10..30}";
            assert_eq!("{", &content[0..1]);
            assert_eq!("10", &content[1..3]);
            assert_eq!("..", &content[3..5]);
            assert_eq!("30", &content[5..7]);
            assert_eq!("}", &content[7..8]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::OBra, 0, 1),
                Token::from_start_end(TokenKind::Number, 1, 3),
                Token::from_start_end(TokenKind::Range, 3, 5),
                Token::from_start_end(TokenKind::Number, 5, 7),
                Token::from_start_end(TokenKind::CBra, 7, 8),
            ];
            the_rest(content, expected_tokens);
        }
    }

    mod special_chars_outside_braces_should_not_be_a_problem {
        use super::*;
        #[test]
        fn comma_outside_braces() {
            let content = "Welcome, {apple,banana123}!";
            assert_eq!("Welcome, ", &content[0..9]);
            assert_eq!("{", &content[9..10]);
            assert_eq!("apple", &content[10..15]);
            assert_eq!(",", &content[15..16]);
            assert_eq!("banana123", &content[16..25]);
            assert_eq!("}", &content[25..26]);
            assert_eq!("!", &content[26..27]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::Text, 0, 9),
                Token::from_start_end(TokenKind::OBra, 9, 10),
                Token::from_start_end(TokenKind::Text, 10, 15),
                Token::from_start_end(TokenKind::Comma, 15, 16),
                Token::from_start_end(TokenKind::Text, 16, 25),
                Token::from_start_end(TokenKind::CBra, 25, 26),
                Token::from_start_end(TokenKind::Text, 26, 27),
            ];
            the_rest(content, expected_tokens);
        }
        #[test]
        fn cbra_outside_braces() {
            let content = "} Welcome, dear child.";
            assert_eq!("} Welcome, dear child.", &content[0..22]);
            let expected_tokens = vec![Token::from_start_end(TokenKind::Text, 0, 22)];
            the_rest(content, expected_tokens);
        }
        #[test]
        fn obra_and_cbra_outside_braces() {
            let content = "These are '%{' and '%}' {opening,closing} curly brackets.";
            assert_eq!("These are '", &content[0..11]);
            assert_eq!("{' and '", &content[12..20]);
            assert_eq!("}' ", &content[21..24]);
            assert_eq!("{", &content[24..25]);
            assert_eq!("opening", &content[25..32]);
            assert_eq!(",", &content[32..33]);
            assert_eq!("closing", &content[33..40]);
            assert_eq!("}", &content[40..41]);
            assert_eq!(" curly brackets.", &content[41..57]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::Text, 0, 11),
                Token::from_start_end(TokenKind::Text, 12, 20),
                Token::from_start_end(TokenKind::Text, 21, 24),
                Token::from_start_end(TokenKind::OBra, 24, 25),
                Token::from_start_end(TokenKind::Text, 25, 32),
                Token::from_start_end(TokenKind::Comma, 32, 33),
                Token::from_start_end(TokenKind::Text, 33, 40),
                Token::from_start_end(TokenKind::CBra, 40, 41),
                Token::from_start_end(TokenKind::Text, 41, 57),
            ];
            the_rest(content, expected_tokens);
        }
        #[test]
        #[cfg(any(
            feature = "numeric_range",
            feature = "char_range",
            feature = "emoji_range"
        ))]
        fn dot_outside_braces() {
            let content = "Prof. {J{ack,ohn},A{lex,dam}}";
            assert_eq!("Prof. ", &content[0..6]);
            assert_eq!("{", &content[6..7]);
            assert_eq!("J", &content[7..8]);
            assert_eq!("{", &content[8..9]);
            assert_eq!("ack", &content[9..12]);
            assert_eq!(",", &content[12..13]);
            assert_eq!("ohn", &content[13..16]);
            assert_eq!("}", &content[16..17]);
            assert_eq!(",", &content[17..18]);
            assert_eq!("A", &content[18..19]);
            assert_eq!("{", &content[19..20]);
            assert_eq!("lex", &content[20..23]);
            assert_eq!(",", &content[23..24]);
            assert_eq!("dam", &content[24..27]);
            assert_eq!("}", &content[27..28]);
            assert_eq!("}", &content[28..29]);
            let expected_tokens = vec![
                Token::from_start_end(TokenKind::Text, 0, 6),
                Token::from_start_end(TokenKind::OBra, 6, 7),
                Token::from_start_end(TokenKind::Text, 7, 8),
                Token::from_start_end(TokenKind::OBra, 8, 9),
                Token::from_start_end(TokenKind::Text, 9, 12),
                Token::from_start_end(TokenKind::Comma, 12, 13),
                Token::from_start_end(TokenKind::Text, 13, 16),
                Token::from_start_end(TokenKind::CBra, 16, 17),
                Token::from_start_end(TokenKind::Comma, 17, 18),
                Token::from_start_end(TokenKind::Text, 18, 19),
                Token::from_start_end(TokenKind::OBra, 19, 20),
                Token::from_start_end(TokenKind::Text, 20, 23),
                Token::from_start_end(TokenKind::Comma, 23, 24),
                Token::from_start_end(TokenKind::Text, 24, 27),
                Token::from_start_end(TokenKind::CBra, 27, 28),
                Token::from_start_end(TokenKind::CBra, 28, 29),
            ];
            the_rest(content, expected_tokens);
        }
    }
}
