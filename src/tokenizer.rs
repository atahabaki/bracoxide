use std::ops::Range;

use crate::flag::Flag;

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

#[allow(clippy::redundant_pub_crate)]
pub(crate) struct Token {
    kind: TokenKind,
    range: Range<usize>,
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
    pub fn get_state(&self) -> BufferState {
        self.previous_buffer_state
    }
    pub fn is_escape(&self) -> bool {
        self.previous_buffer_state == BufferState::Escape
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
        self.range = Range {
            start: self.range.start,
            end: self.range.end + 1,
        };
    }
    pub fn decrement_range_end(&mut self) {
        self.range = Range {
            start: self.range.start,
            end: self.range.end - 1,
        };
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
        let mut vec = vec![];
        let mut iter = data.chars().enumerate();
        let escape_char = self.flags.escape_char;
        // let mut is_escape = false;
        // // Respectively count of opening bracket, closing  bracket
        // let mut count = (0_usize, 0_usize);
        let mut state = TokenizerState::default();
        while let Some((i, c)) = iter.next() {
            match c {
                _ if c == escape_char => {
                    match state.get_state() {
                        BufferState::Escape => {
                            todo!()
                            // First option: consider this one the text, add token, start range from here
                            // Second option: consider the previous one as text, add token etc.
                            // Second option is less-work i suppose.
                        }
                        BufferState::Text => {
                            // change range's start pos
                            // add token.
                            todo!()
                        }
                        BufferState::Number => {
                            // change range's start pos
                            // add token
                            todo!()
                        }
                    }
                    state.set_escape(true);
                }
                _ if state.is_escape() => {
                    state.set_escape(false);
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
                _ => {}
            }
        }
        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn special_chars_outside_braces_should_not_be_a_problem() {
        todo!("Special chars outside curly braces should not throw Err")
    }
}
