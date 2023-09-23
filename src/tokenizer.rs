/*
 * This file is part of bracoxide.
 *
 * bracoxide is under MIT license.
 *
 * Copyright (c) 2023 A. Taha Baki <atahabaki@pm.me>
 */

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum TokenKind {
    OpeningBracket,
    ClosingBracket,
    Comma,
    /// Text length
    Text(usize),
    /// Number's length
    Number(usize),
    Range,
    /// length
    Empty(usize),
}

impl TokenKind {
    pub fn get_length(&self) -> usize {
        match self {
            TokenKind::Empty(l) | TokenKind::Number(l) | TokenKind::Text(l) => *l,
            TokenKind::Range => 2,
            _ => 1,
        }
    }
    pub fn next_position(&self, current: usize) -> usize {
        current + self.get_length()
    }
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::OpeningBracket => write!(f, "Opening"),
            TokenKind::ClosingBracket => write!(f, "Closing"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Text(l) => write!(f, "Text, len: {}", l),
            TokenKind::Number(l) => write!(f, "Number, len: {}", l),
            TokenKind::Empty(l) => write!(f, "Empty, len: {}", l),
            TokenKind::Range => write!(f, "Range"),
        }
    }
}

pub type TokenMap = std::collections::HashMap<usize, TokenKind>;

#[derive(Default, PartialEq)]
#[cfg_attr(test, derive(Debug))]
enum State {
    Escape,
    Comma,
    Opening,
    Closing,
    Text,
    Number,
    #[default]
    None,
}

/// Position start and length.
type Cut = (usize, usize);

trait StartPosition<T> {
    fn start_position(&mut self, position: T);
}

impl StartPosition<usize> for Cut {
    fn start_position(&mut self, position: usize) {
        *self = (position, 1);
    }
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[cfg_attr(feature = "simplerr", derive(Debug))]
pub enum TokenizationError {
    NoContent,
    EmptyBraces,
    BracesDontMatch,
    NoBraces,
    NothingToEscape,
}

impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizationError::NoContent => write!(f, "No content to tokenize from."),
            TokenizationError::EmptyBraces => write!(
                f,
                "Empty braces increases loop count. Remove empty braces ('{{}}')."
            ),
            TokenizationError::BracesDontMatch => {
                write!(f, "Opening and closing brackets' count does not match.")
            }
            TokenizationError::NoBraces => write!(f, "Not a single brace found."),
            TokenizationError::NothingToEscape => write!(
                f,
                "Escape character ('\\') used but there's nothing to escape."
            ),
        }
    }
}

#[cfg(feature = "simplerr")]
impl std::error::Error for TokenizationError {}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub struct Tokenizer<'a> {
    content: &'a str,
    state: State,
    text_cut: Cut,
    number_cut: Cut,
    /// Counts of opening and closing bracket.
    count: Cut,
    /// token beginning position -> TokenKind
    pub tokens: TokenMap,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Result<Self, TokenizationError> {
        if content.is_empty() {
            return Err(TokenizationError::NoContent);
        }
        Ok(Tokenizer {
            content,
            tokens: TokenMap::new(),
            text_cut: (0, 0),
            number_cut: (0, 0),
            count: (0, 0),
            state: State::default(),
        })
    }
    fn insert_token(&mut self, position: usize, kind: TokenKind) {
        self.tokens.insert(position, kind);
    }
    fn tokenize_number(&mut self) {
        if self.number_cut.1 > 0 {
            self.insert_token(self.number_cut.0, TokenKind::Number(self.number_cut.1));
            self.number_cut = (0, 0);
        }
    }
    fn tokenize_text(&mut self) {
        if self.text_cut.1 > 0 {
            self.insert_token(self.text_cut.0, TokenKind::Text(self.text_cut.1));
            self.text_cut = (0, 0);
        }
    }
    fn tokenize_buffers(&mut self) {
        self.tokenize_text();
        self.tokenize_number();
    }
    fn text_start(&mut self, position: usize) {
        self.tokenize_number();
        self.text_cut.start_position(position);
        self.state = State::Text;
    }
    fn number_start(&mut self, position: usize) {
        self.tokenize_text();
        self.number_cut.start_position(position);
        self.state = State::Number;
    }
    fn insert_opening(&mut self, position: usize) {
        self.count.0 += 1;
        self.state = State::Opening;
        self.insert_token(position, TokenKind::OpeningBracket);
    }
    fn insert_closing(&mut self, position: usize) {
        self.count.1 += 1;
        self.state = State::Closing;
        self.insert_token(position, TokenKind::ClosingBracket);
    }
    pub fn get_content(&self) -> &'a str {
        self.content
    }
    pub fn tokenize(&mut self) -> Result<(), TokenizationError> {
        let mut iter = self.content.chars().enumerate();
        'tokenize: while let Some((i, c)) = iter.next() {
            match (&self.state, c) {
                (State::Escape, _) => self.text_start(i),
                (_, '\\') => {
                    self.tokenize_buffers();
                    self.state = State::Escape;
                }
                (State::Number, '0'..='9') => self.number_cut.1 += 1,
                (_, '0'..='9') => self.number_start(i),
                (State::Text, '.') => self.text_cut.1 += 1,
                (State::None | State::Number, '.') => {
                    self.tokenize_number();
                    let mut check = iter.clone();
                    if let Some((_, nc)) = check.next() {
                        match nc {
                            '.' => {
                                self.insert_token(i, TokenKind::Range);
                                iter = check;
                                self.state = State::None;
                                continue;
                            }
                            // support for floats?
                            // '0'..='9' => todo!(),
                            _ => self.text_start(i),
                        }
                    } else {
                        self.insert_token(i, TokenKind::Text(1));
                    }
                }
                (_, '.') => self.text_start(i),
                (_, '{') => {
                    self.tokenize_buffers();
                    self.insert_opening(i);
                }

                (State::Opening, '}') => return Err(TokenizationError::EmptyBraces),
                (_, '}') => {
                    self.tokenize_buffers();
                    self.insert_closing(i);
                }
                (old_state, ',') => {
                    let was_opening = old_state == &State::Opening;
                    if (self.count.0 == 0 || self.count.0 == self.count.1) && !was_opening {
                        // w- escaping: `{A,B,C},D` -> [`A,D`, `B,D`, `C,D`]
                        // w/ escaping: `{A,B,C}\,D` -> [`A,D`, `B,D`, `C,D`]
                        if self.text_cut.1 >= 1 {
                            self.text_cut.1 += 1;
                        } else {
                            self.tokenize_buffers();
                            self.text_start(i);
                        }
                    } else {
                        // HOW:
                        // 1. if the previous token was `{` or
                        // 2. if the count of consecutive commas (i.e. `,,,,`) are > 1
                        // 3. if the next token is `}` then its empty token.
                        // otherwise it is normal comma.
                        // PR, when you find a better algorithm.
                        self.tokenize_buffers();
                        let mut comma_count = 1_usize;
                        let mut counter = iter.clone();
                        let mut prev_iter = iter.clone();
                        'commacounter: while let Some((ni, nc)) = counter.next() {
                            match nc {
                                ',' => {
                                    comma_count += 1;
                                    iter = counter.clone();
                                }
                                '}' => {
                                    self.insert_token(i, TokenKind::Empty(comma_count));
                                    self.insert_closing(ni);
                                    iter = counter.clone();
                                    continue 'tokenize;
                                }
                                _ => {
                                    iter = prev_iter;
                                    break 'commacounter;
                                }
                            }
                            prev_iter = counter.clone();
                        }
                        match comma_count > 1 || was_opening {
                            true => self.insert_token(i, TokenKind::Empty(comma_count)),
                            _ => {
                                self.insert_token(i, TokenKind::Comma);
                            }
                        }
                        self.state = State::Comma;
                    }
                }
                (State::Text, _) => self.text_cut.1 += 1,
                (_, _) => {
                    self.tokenize_buffers();
                    self.text_start(i);
                }
            }
        }
        self.tokenize_buffers();
        if self.state == State::Escape {
            return Err(TokenizationError::NothingToEscape);
        }
        if self.count == (0, 0) {
            return Err(TokenizationError::NoBraces);
        }
        if self.count.0 != self.count.1 {
            return Err(TokenizationError::BracesDontMatch);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_empty_content_returns_nocontent() {
        assert_eq!(Tokenizer::new(""), Err(TokenizationError::NoContent));
    }

    #[test]
    fn test_escape_misuse() {
        let mut tokenizer = Tokenizer::new("\\").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::NothingToEscape)
        );
        let mut tokenizer = Tokenizer::new(" \\").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::NothingToEscape)
        );
    }

    #[test]
    fn test_no_braces_used() {
        let mut tokenizer = Tokenizer::new("10801920").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::NoBraces));
        let mut tokenizer = Tokenizer::new("Salut!\\, mon ami!").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::NoBraces));
    }

    #[test]
    fn test_empty_braces_returns_empty_braces() {
        let mut tokenizer = Tokenizer::new("{}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::EmptyBraces));
        let mut tokenizer = Tokenizer::new("{{}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::EmptyBraces));
        let mut tokenizer = Tokenizer::new("{}}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::EmptyBraces));
    }

    #[test]
    fn test_braces_dont_match() {
        let mut tokenizer = Tokenizer::new("{").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::BracesDontMatch)
        );
        let mut tokenizer = Tokenizer::new("}").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::BracesDontMatch)
        );
        let mut tokenizer = Tokenizer::new("{A}}").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::BracesDontMatch)
        );
        let mut tokenizer = Tokenizer::new("{{A}").unwrap();
        assert_eq!(
            tokenizer.tokenize(),
            Err(TokenizationError::BracesDontMatch)
        );
    }

    #[test]
    fn test_simple_number() {
        let mut tokenizer = Tokenizer::new("{10801920}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Number(8));
        expected_map.insert(9, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_simple_text() {
        let mut tokenizer = Tokenizer::new("{Salut\\, mon ami!}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Text(5));
        expected_map.insert(7, TokenKind::Text(10));
        expected_map.insert(17, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_simple_collection() {
        let mut tokenizer = Tokenizer::new("{A,B}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Text(1));
        expected_map.insert(2, TokenKind::Comma);
        expected_map.insert(3, TokenKind::Text(1));
        expected_map.insert(4, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_simple_range() {
        let mut tokenizer = Tokenizer::new("{3..5}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Number(1));
        expected_map.insert(2, TokenKind::Range);
        expected_map.insert(4, TokenKind::Number(1));
        expected_map.insert(5, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_annoying_dots1() {
        let mut tokenizer = Tokenizer::new("{1.2.3,b}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Number(1));
        expected_map.insert(2, TokenKind::Text(1));
        expected_map.insert(3, TokenKind::Number(1));
        expected_map.insert(4, TokenKind::Text(1));
        expected_map.insert(5, TokenKind::Number(1));
        expected_map.insert(6, TokenKind::Comma);
        expected_map.insert(7, TokenKind::Text(1));
        expected_map.insert(8, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokenizer.tokens)
    }

    #[test]
    fn test_annoying_dots2() {
        let mut tokenizer = Tokenizer::new("{a.b.c,d}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Text(5));
        expected_map.insert(6, TokenKind::Comma);
        expected_map.insert(7, TokenKind::Text(1));
        expected_map.insert(8, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokenizer.tokens)
    }

    #[test]
    fn test_annoying_dots3() {
        let mut tokenizer = Tokenizer::new("{a.1.c.2.d,3.e.4,f}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::OpeningBracket);
        expected_map.insert(1, TokenKind::Text(2));
        expected_map.insert(3, TokenKind::Number(1));
        expected_map.insert(4, TokenKind::Text(3));
        expected_map.insert(7, TokenKind::Number(1));
        expected_map.insert(8, TokenKind::Text(2));
        expected_map.insert(10, TokenKind::Comma);
        expected_map.insert(11, TokenKind::Number(1));
        expected_map.insert(12, TokenKind::Text(3));
        expected_map.insert(15, TokenKind::Number(1));
        expected_map.insert(16, TokenKind::Comma);
        expected_map.insert(17, TokenKind::Text(1));
        expected_map.insert(18, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokenizer.tokens)
    }

    #[test]
    fn test_simple_expansion() {
        let mut tokenizer = Tokenizer::new("A{B,C}D{13..25}").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Text(1));
        expected_map.insert(1, TokenKind::OpeningBracket);
        expected_map.insert(2, TokenKind::Text(1));
        expected_map.insert(3, TokenKind::Comma);
        expected_map.insert(4, TokenKind::Text(1));
        expected_map.insert(5, TokenKind::ClosingBracket);
        expected_map.insert(6, TokenKind::Text(1));
        expected_map.insert(7, TokenKind::OpeningBracket);
        expected_map.insert(8, TokenKind::Number(2));
        expected_map.insert(10, TokenKind::Range);
        expected_map.insert(12, TokenKind::Number(2));
        expected_map.insert(14, TokenKind::ClosingBracket);
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_empty_start() {
        let mut tokenizer = Tokenizer::new("A{,B,C}D").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Text(1));
        expected_map.insert(1, TokenKind::OpeningBracket);
        expected_map.insert(2, TokenKind::Empty(1));
        expected_map.insert(3, TokenKind::Text(1));
        expected_map.insert(4, TokenKind::Comma);
        expected_map.insert(5, TokenKind::Text(1));
        expected_map.insert(6, TokenKind::ClosingBracket);
        expected_map.insert(7, TokenKind::Text(1));
        assert_eq!(expected_map, tokenizer.tokens)
    }

    #[test]
    fn test_empty_middle() {
        let mut tokenizer = Tokenizer::new("A{B,,C}D").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Text(1));
        expected_map.insert(1, TokenKind::OpeningBracket);
        expected_map.insert(2, TokenKind::Text(1));
        expected_map.insert(3, TokenKind::Empty(2));
        expected_map.insert(5, TokenKind::Text(1));
        expected_map.insert(6, TokenKind::ClosingBracket);
        expected_map.insert(7, TokenKind::Text(1));
        assert_eq!(expected_map, tokenizer.tokens)
    }

    #[test]
    fn test_empty_end() {
        let mut tokenizer = Tokenizer::new("A{B,C,}D").unwrap();
        assert_eq!(tokenizer.tokenize(), Ok(()));
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Text(1));
        expected_map.insert(1, TokenKind::OpeningBracket);
        expected_map.insert(2, TokenKind::Text(1));
        expected_map.insert(3, TokenKind::Comma);
        expected_map.insert(4, TokenKind::Text(1));
        expected_map.insert(5, TokenKind::Empty(1));
        expected_map.insert(6, TokenKind::ClosingBracket);
        expected_map.insert(7, TokenKind::Text(1));
        assert_eq!(expected_map, tokenizer.tokens)
    }
}
