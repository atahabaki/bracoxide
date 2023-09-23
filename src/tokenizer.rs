/*
 * This file is part of bracoxide.
 *
 * bracoxide is under MIT license.
 *
 * Copyright (c) 2023 A. Taha Baki <atahabaki@pm.me>
 */

use std::collections::HashMap;

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
    Empty
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
enum State {
    Escape,
    Comma,
    Opening,
    Closing,
    Text,
    Number,
    None,
}

impl Default for State {
    fn default() -> Self {
        State::None
    }
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
#[cfg_attr(test,derive(Debug))]
#[cfg_attr(feature="simplerr", derive(Debug))]
pub enum TokenizationError {
    NoContent,
    EmptyBraces,
    BracesDontMatch
}

impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizationError::NoContent => write!(f, "No content to tokenize from."),
            TokenizationError::EmptyBraces => write!(f, "Empty braces increases loop count. Remove empty braces ('{{}}')."),
            TokenizationError::BracesDontMatch => write!(f, "Opening and closing brackets' count does not match."),
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
    pub tokens: HashMap<usize, TokenKind>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Result<Self, TokenizationError> {
        if content.is_empty() {
            return Err(TokenizationError::NoContent);
        }
        Ok(Tokenizer {
            content,
            tokens: HashMap::new(),
            text_cut: (0,0),
            number_cut: (0,0),
            count: (0,0),
            state: State::default()
        })
    }
    fn insert_token(&mut self, position: usize, kind: TokenKind) {
        self.tokens.insert(position, kind);
    }
    fn tokenize_number(&mut self) {
        if self.number_cut.1 > 0 {
            self.insert_token(self.number_cut.0, TokenKind::Number(self.number_cut.1));
            self.number_cut = (0,0);
        }
    }
    fn tokenize_text(&mut self) {
        if self.text_cut.1 > 0 {
            self.insert_token(self.text_cut.0, TokenKind::Text(self.text_cut.1));
            self.text_cut = (0,0);
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
    pub fn tokenize(&mut self) -> Result<(), TokenizationError>{
        let mut iter = self.content.chars().enumerate();
        while let Some((i, c)) = iter.next() {
            match (&self.state, c) {
                (State::Escape, _) => self.text_start(i),
                (old_state, '\\') => {
                    match old_state {
                        State::Comma => self.insert_token(i-1, TokenKind::Comma),
                        State::Text => self.tokenize_text(),
                        State::Number => self.tokenize_number(),
                        State::Closing |
                        State::Opening |
                        State::None => (),
                        State::Escape => unreachable!(),
                    }
                    self.state = State::Escape;
                }

                (State::Number, '0'..='9') => self.number_cut.1 += 1,
                (State::Comma, '0'..='9') => {
                    self.insert_token(i, TokenKind::Comma);
                    self.number_start(i);
                }
                (_, '0'..='9') => 
                self.number_start(i),
                (old_state, '.') => {
                    let mut skip = false;
                    match old_state {
                        // Range without starting limit. Defaults to zero.
                        State::None |
                        State::Number => {
                            self.tokenize_number();
                            let mut check = iter.clone();
                            if let Some((_ni, nc)) = check.next() {
                                match nc {
                                    '.' => {
                                        self.insert_token(i, TokenKind::Range);
                                        iter = check;
                                        self.state = State::None;
                                        skip = true;
                                    }
                                    // support for floats?
                                    // '0'..='9' => todo!(),
                                    _ => self.text_start(i),
                                }
                            } else {
                                self.insert_token(i, TokenKind::Text(1));
                            }
                        },
                        // continue text...
                        State::Text => {self.text_cut.1+=1; continue;}
                        State::Comma => self.insert_token(i-1, TokenKind::Comma),
                        // it is definitelly not a Range, so count as text.
                        State::Opening |
                        State::Closing => (),
                        State::Escape => unreachable!(),
                    }
                    if !skip {
                        self.text_start(i);
                    }
                }
                (old_state, '{') => {
                    match old_state {
                        State::Comma => self.insert_token(i-1, TokenKind::Comma),
                        State::Text => self.tokenize_text(),
                        State::Number => self.tokenize_number(),
                        State::Opening |
                        State::Closing |
                        State::None => (),
                        State::Escape => unreachable!(),
                    }
                    self.insert_opening(i);
                }
                (old_state, '}') => {
                    match old_state {
                        State::Comma => self.insert_token(i-1, TokenKind::Comma),
                        // Return error:
                        // Case: {}, completely empty braces.
                        State::Opening => return Err(TokenizationError::EmptyBraces),
                        State::Text => self.tokenize_text(),
                        State::Number => self.tokenize_number(),
                        State::Closing |
                        State::None => (),
                        State::Escape => unreachable!(),
                    }
                    self.insert_closing(i);
                }
                (old_state, ',') => {
                    match old_state {
                        State::Opening |
                        State::Comma => {
                            self.insert_token(i, TokenKind::Empty);
                            continue;
                        }
                        State::Text => self.tokenize_text(),
                        State::Number => self.tokenize_number(),
                        State::Closing |
                        State::None => (),
                        State::Escape => unreachable!(),
                    }
                    self.insert_token(i, TokenKind::Comma);
                    self.state = State::Comma;
                }
                (State::Comma, _) => {
                    self.insert_token(i-1, TokenKind::Comma);
                    self.text_start(i);
                }
                (old_state,_) => {
                    match old_state {
                        State::Number => self.tokenize_number(),
                        State::Comma => self.insert_token(i-1, TokenKind::Comma),
                        State::Text => {
                            self.text_cut.1 += 1;
                            continue;
                        }
                        State::Opening |
                        State::Closing |
                        State::None => (),
                        State::Escape => unreachable!(),
                    }
                    self.text_start(i);
                }
            }
        }
        self.tokenize_buffers();
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
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::BracesDontMatch));
        let mut tokenizer = Tokenizer::new("}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::BracesDontMatch));
        let mut tokenizer = Tokenizer::new("{A}}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::BracesDontMatch));
        let mut tokenizer = Tokenizer::new("{{A}").unwrap();
        assert_eq!(tokenizer.tokenize(), Err(TokenizationError::BracesDontMatch));
    }

    #[test]
    fn test_simple_number() {
        let mut tokenizer = Tokenizer::new("10801920").unwrap();
        assert_eq!(tokenizer.tokenize(),  Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Number(8));
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_simple_text() {
        let mut tokenizer = Tokenizer::new("Salut\\, mon ami!").unwrap();
        assert_eq!(tokenizer.tokenize(),  Ok(()));
        let tokens = tokenizer.tokens;
        let mut expected_map = HashMap::<usize, TokenKind>::new();
        expected_map.insert(0, TokenKind::Text(5));
        expected_map.insert(6, TokenKind::Text(10));
        assert_eq!(expected_map, tokens)
    }

    #[test]
    fn test_simple_collection() {
        let mut tokenizer = Tokenizer::new("{A,B}").unwrap();
        assert_eq!(tokenizer.tokenize(),  Ok(()));
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
        assert_eq!(tokenizer.tokenize(),  Ok(()));
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
        assert_eq!(tokenizer.tokenize(),  Ok(()));
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
}