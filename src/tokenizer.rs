use std::ops::Range;

use crate::flag::Flag;

#[derive(Debug, PartialEq)]
pub(crate) enum TokenKind {
    OBra,
    CBra,
    Comma,
    #[cfg(feature = "variable")]
    Colon,
    #[cfg(any(feature = "arithmetic_range", feature = "range_padding"))]
    Semicolon,
    #[cfg(feature = "range_padding")]
    Equal,
    #[cfg(feature = "arithmetic_range")]
    Plus,
    #[cfg(feature = "arithmetic_range")]
    Minus,
    #[cfg(feature = "char_range")]
    Char,
    Text,
    Number,
}

#[derive(Debug, PartialEq)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Token {
    kind: TokenKind,
    range: Range<usize>,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self { kind, range }
    }
    pub(crate) fn from_start_end(kind: TokenKind, range_start: usize, range_end: usize) -> Self {
        Self {
            kind,
            range: Range {
                start: range_start,
                end: range_end,
            },
        }
    }
}

#[derive(Debug)]
#[allow(clippy::redundant_pub_crate)]
pub(crate) enum TokenizerError {
    NoData,
}

impl std::error::Error for TokenizerError {}
impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "Data is empty"),
        }
    }
}

pub(crate) struct Tokenizer<'a> {
    pub(crate) data: &'a str,
    pub(crate) flags: Flag,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum BufferState {
    // None,
    Escape,
    #[default]
    Text,
    Number,
    TokenPushed,
}

struct TokenizerState {
    // Opening bracket and closing bracket count, respectively
    count: (usize, usize),
    // it is actually mix of current state & previous state.
    previous_buffer_state: BufferState,
    range: Range<usize>,
}

impl Default for TokenizerState {
    fn default() -> Self {
        Self {
            count: (0, 0), // It kinda  looks like an owl. Hi, owl>
            previous_buffer_state: BufferState::default(),
            range: Range { start: 0, end: 0 },
        }
    }
}

impl TokenizerState {
    pub fn get_previous_buffer_state(&self) -> BufferState {
        self.previous_buffer_state
    }
    pub fn is_escape(&self) -> bool {
        self.previous_buffer_state == BufferState::Escape
    }
    pub fn set_state_number(&mut self) {
        self.previous_buffer_state = BufferState::Number
    }
    pub fn set_state_text(&mut self) {
        self.previous_buffer_state = BufferState::Text
    }
    pub fn set_state_token(&mut self) {
        self.previous_buffer_state = BufferState::TokenPushed
    }
    pub fn set_escape(&mut self, is_escape: bool) {
        if is_escape {
            self.previous_buffer_state = BufferState::Escape;
        } else {
            // I can not imagine any other rational stiuations than:
            // Feel free to create another example, that breaks this mindset?
            // "These are called '\{', '\}' {Opening,Closing} bracket"
            // See, I could escape a char inside curly braces but that will be a char no matter what, so the next state
            // certainly be text.
            self.previous_buffer_state = BufferState::default();
        }
    }
    pub fn is_inside_curly_brackets(&self) -> bool {
        self.count.0 > self.count.1
    }
    pub fn increment_obra(&mut self) {
        self.count.0 += 1;
    }
    pub fn increment_cbra(&mut self) {
        self.count.1 += 1;
    }
    pub fn increment_range_end(&mut self) {
        self.range.end = self.range.end + 1;
    }
    pub fn decrement_range_end(&mut self) {
        self.range.end = self.range.end - 1;
    }
    pub fn new_range(&mut self, start: usize) {
        self.range = Range {
            start,
            end: start + 1,
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub(crate) const fn new(data: &'a str, flags: Flag) -> Self {
        Self { data, flags }
    }

    pub(crate) fn tokenize(&self) -> Result<Vec<Token>, TokenizerError> {
        let data = self.data.to_string();
        if data.is_empty() {
            return Err(TokenizerError::NoData);
        }
        let mut tokens = vec![];
        let mut iter = data.chars().enumerate();
        let escape_char = self.flags.escape_char;
        let suppress_warning = self.flags.supress_warning;
        let mut state = TokenizerState::default();
        while let Some((i, c)) = iter.next() {
            match c {
                // Previously tokenizer met with actual escape char
                _ if state.is_escape() => {
                    // I can not see any particular reason to use escape char to
                    // escape not special characters so, im making a choice here
                    // escape char is only meant to escape special chars treated as text.
                    state.new_range(i);
                    match c {
                        _ if c == escape_char => {
                            state.set_state_text();
                        }
                        '{' | ',' | '}' => {
                            state.set_state_text();
                        }
                        // TODO: Gotta throw an error or warning message
                        _ if suppress_warning => (),
                        _ => {}
                    }
                    state.set_escape(false);
                }
                // tokenizer met with actual escape char
                _ if c == escape_char => {
                    // 1. push token based on the previous buffer state
                    state.increment_range_end();
                    match state.previous_buffer_state {
                        BufferState::Escape => unreachable!(),
                        BufferState::Text => {
                            let token = Token::new(TokenKind::Text, state.range.clone());
                            tokens.push(token);
                        }
                        BufferState::Number => {
                            let token = Token::new(TokenKind::Number, state.range.clone());
                            tokens.push(token);
                        }
                        // it is sth. like
                        // {a,%
                        //    ^
                        // What would you do, start a new range?
                        BufferState::TokenPushed => (),
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
                _ if c.is_numeric() => {}
                _ => {
                    state.increment_range_end();
                }
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenizer_state_inc_dec() {
        let mut state = TokenizerState::default();
        state.increment_range_end();
        let expected_range = Range { start: 0, end: 1 };
        assert_eq!(state.range, expected_range);
        state.decrement_range_end();
        let expected_range = Range { start: 0, end: 0 };
        assert_eq!(state.range, expected_range);
    }

    fn the_rest(content: &str, expected_tokens: Vec<Token>) {
        let tokenizer = Tokenizer::new(content, Flag::default());
        let tokens = tokenizer.tokenize();
        assert!(tokens.is_ok());
        assert_eq!(tokens.unwrap(), expected_tokens);
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
            Token::from_start_end(TokenKind::Text, 7, 8), // '%'
            Token::from_start_end(TokenKind::Number, 8, 12), // '1345'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn number_text() {
        let content = "1345banana";
        let expected_tokens = vec![
            Token::from_start_end(TokenKind::Number, 0, 4), // '1345'
            Token::from_start_end(TokenKind::Text, 4, 10),  // 'banana'
        ];
        the_rest(content, expected_tokens);
    }
    #[test]
    fn special_chars_outside_braces_should_not_be_a_problem() {
        todo!("Special chars outside curly braces should not throw Err")
    }
}
