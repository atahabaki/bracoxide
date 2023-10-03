/*
 * This file is part of bracoxide.
 *
 * bracoxide is under MIT license.
 *
 * Copyright (c) 2023 A. Taha Baki <atahabaki@pm.me>
 */

use crate::tokenizer::*;

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[cfg_attr(feature = "simplerr", derive(Debug))]
pub enum ParsingError {
    NoContent,
    NoTokens,
    NoFragment,
    ExtraOpeningBracket(usize),
    ExtraClosingBracket(usize),
    OpeningBracketExpected(usize),
    NoCommaInRange(usize),
    NoTextInRange(usize),
    ExtraRange(usize),
    ExpectedText(usize),
    StartLimitExpected(usize),
    EndLimitExpected(usize),
    NothingInBraces(usize),
}

#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
pub enum Node {
    Text {
        content: String,
        #[cfg(test)]
        start: usize,
        #[cfg(test)]
        end: usize,
    },
    BraceExpansion {
        prefix: Option<Box<Node>>,
        inside: Option<Box<Node>>,
        postfix: Option<Box<Node>>,
        #[cfg(test)]
        start: usize,
        #[cfg(test)]
        end: usize,
    },
    Collection {
        items: Vec<Node>,
        #[cfg(test)]
        start: usize,
        #[cfg(test)]
        end: usize,
    },
    Range {
        from: String,
        to: String,
        #[cfg(test)]
        start: usize,
        #[cfg(test)]
        end: usize,
    },
}

pub struct Parser<'a> {
    _content: &'a str,
    tokens: TokenMap,
}

pub(crate) type _Fragment = Vec<usize>;
pub(crate) type _Fragments = (Option<_Fragment>, Option<_Fragment>, Option<_Fragment>);

impl<'a> Parser<'a> {
    pub fn from_tokenizer(tokenizer: Tokenizer<'a>) -> Result<Self, ParsingError> {
        if tokenizer.tokens.is_empty() {
            return Err(ParsingError::NoTokens);
        }
        Ok(Parser {
            _content: tokenizer.get_content(),
            tokens: tokenizer.tokens,
        })
    }

    pub fn new(content: &'a str, tokens: TokenMap) -> Result<Self, ParsingError> {
        if content.is_empty() {
            return Err(ParsingError::NoContent);
        }
        if tokens.is_empty() {
            return Err(ParsingError::NoTokens);
        }
        Ok(Parser {
            _content: content,
            tokens,
        })
    }

    fn get_a_slice_of_cake(&self, start: usize, end: usize) -> String {
        self._content
            .chars()
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn parse(&self) -> Result<Node, ParsingError> {
        let mut keys: Vec<usize> = self.tokens.keys().cloned().collect();
        keys.sort();
        self.reparse(&keys)
    }

    pub(crate) fn reparse(&self, fragment: &Vec<usize>) -> Result<Node, ParsingError> {
        if fragment.is_empty() {
            return Err(ParsingError::NoFragment);
        }
        match self.seperate(fragment) {
            Ok(seperated) => {
                let prefix = if let Some(prefix) = seperated.0 {
                    match self.text(&prefix) {
                        Ok(n) => Some(Box::new(n)),
                        Err(e) => return Err(e),
                    }
                } else {
                    None
                };
                let inside = if let Some(inside) = seperated.1 {
                    match self.collection(&inside) {
                        Ok(n) => Some(Box::new(n)),
                        Err(e) => return Err(e),
                    }
                } else {
                    None
                };
                let postfix = if let Some(postfix) = seperated.2 {
                    let parsed = if postfix.iter().any(|ti| {
                        matches!(
                            self.tokens.get(ti).unwrap(),
                            TokenKind::OpeningBracket | TokenKind::ClosingBracket
                        )
                    }) {
                        self.reparse(&postfix)
                    } else {
                        self.text(&postfix)
                    };
                    match parsed {
                        Ok(n) => Some(Box::new(n)),
                        Err(e) => return Err(e),
                    }
                } else {
                    None
                };
                #[cfg(test)]
                let mut pos = (0_usize, 0_usize);
                #[cfg(test)]
                if let Some(token_index) = fragment.first() {
                    pos.0 = *token_index;
                }
                #[cfg(test)]
                if let Some(token_index) = fragment.last() {
                    pos.1 = *token_index + self.tokens.get(token_index).unwrap().get_length();
                }
                Ok(Node::BraceExpansion {
                    prefix,
                    inside,
                    postfix,
                    #[cfg(test)]
                    start: pos.0,
                    #[cfg(test)]
                    end: pos.1,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub(crate) fn seperate(&self, fragment: &Vec<usize>) -> Result<_Fragments, ParsingError> {
        if fragment.is_empty() {
            return Err(ParsingError::NoFragment);
        }
        #[derive(PartialEq)]
        enum WalkState {
            _Prefix,
            _Inside,
            _Postfix,
        }
        // initialize
        let mut count = (0_usize, 0_usize);
        let mut prefix = vec![];
        let mut inside = vec![];
        let mut postfix = vec![];
        let mut bracing_state = WalkState::_Prefix;
        for token_index in fragment.iter() {
            if let Some(token) = self.tokens.get(token_index) {
                match token {
                    TokenKind::OpeningBracket => {
                        count.0 += 1;
                        match bracing_state {
                            WalkState::_Prefix => {
                                bracing_state = WalkState::_Inside;
                                inside.push(*token_index);
                            }
                            WalkState::_Inside => inside.push(*token_index),
                            WalkState::_Postfix => postfix.push(*token_index),
                        }
                    }
                    TokenKind::ClosingBracket => {
                        count.1 += 1;
                        match bracing_state {
                            WalkState::_Prefix => {
                                return Err(ParsingError::ExtraClosingBracket(*token_index))
                            }
                            WalkState::_Inside => {
                                inside.push(*token_index);
                                if count.0 == count.1 {
                                    bracing_state = WalkState::_Postfix;
                                }
                            }
                            WalkState::_Postfix => postfix.push(*token_index),
                        }
                    }
                    TokenKind::Comma | TokenKind::Range => {
                        return Err(ParsingError::OpeningBracketExpected(*token_index))
                    }
                    _ => match bracing_state {
                        WalkState::_Prefix => prefix.push(*token_index),
                        WalkState::_Inside => inside.push(*token_index),
                        WalkState::_Postfix => postfix.push(*token_index),
                    },
                }
            } else {
                // I don't think this will ever got reach.
                // unless memory written by another program, e.g. CheatEngine
                unreachable!();
            }
        }
        let pre = if prefix.is_empty() {
            None
        } else {
            Some(prefix)
        };
        let ins = if inside.is_empty() {
            None
        } else {
            Some(inside)
        };
        let post = if postfix.is_empty() {
            None
        } else {
            Some(postfix)
        };
        Ok((pre, ins, post))
    }

    pub(crate) fn text(&self, fragment: &Vec<usize>) -> Result<Node, ParsingError> {
        if fragment.is_empty() {
            return Err(ParsingError::NoFragment);
        }
        let mut content = String::new();
        // it is safe to use unwrap here, since we know that
        // fragment is not empty.
        let _start_pos = fragment.first().unwrap();
        for token_index in fragment.iter() {
            if let Some(token) = self.tokens.get(token_index) {
                match token {
                    TokenKind::Text(l) | TokenKind::Number(l) => content.push_str(
                        self.get_a_slice_of_cake(*token_index, *token_index + l)
                            .as_str(),
                    ),
                    _ => return Err(ParsingError::ExpectedText(*token_index)),
                }
            }
        }
        let _len = content.chars().count();
        println!("{}", _len);
        Ok(Node::Text {
            content,
            #[cfg(test)]
            start: *_start_pos,
            #[cfg(test)]
            end: *_start_pos + _len,
        })
    }

    pub(crate) fn collection(&self, fragment: &Vec<usize>) -> Result<Node, ParsingError> {
        if fragment.is_empty() {
            return Err(ParsingError::NoFragment);
        }
        let mut pos = (0_usize, 0_usize);
        let mut count = (0_usize, 0_usize);
        let mut collections: Vec<Vec<usize>> = vec![];
        let mut current = vec![];
        for token_index in fragment.iter() {
            if let Some(token) = self.tokens.get(token_index) {
                match token {
                    TokenKind::Empty(_) if count.0 == (count.1 + 1) => {
                        // inside the parantheses
                        current.push(*token_index);
                        collections.push(current.clone());
                        current.clear();
                    }
                    TokenKind::Comma if count.0 == (count.1 + 1) => {
                        if !current.is_empty() {
                            collections.push(current.clone());
                            current.clear();
                        }
                    }
                    TokenKind::Empty(_) | TokenKind::Comma => current.push(*token_index),
                    TokenKind::OpeningBracket => {
                        if count.0 == 0 {
                            pos.0 = *token_index;
                        } else {
                            current.push(*token_index);
                        }
                        count.0 += 1;
                    }
                    TokenKind::ClosingBracket => {
                        count.1 += 1;
                        if count.0 == count.1 {
                            pos.1 = *token_index;
                        } else {
                            current.push(*token_index);
                        }
                    }
                    _ => current.push(*token_index),
                }
            }
        }
        if !current.is_empty() {
            collections.push(current.clone());
        }
        match collections.len() {
            0 => Err(ParsingError::NothingInBraces(pos.0)),
            1 => {
                // it is absolutely text or range
                // can not be collection.
                let collection = &collections[0];
                match collection
                    .iter()
                    .any(|t| matches!(self.tokens.get(t).unwrap(), TokenKind::Range))
                {
                    true => self.range(collection),
                    false => self.text(collection),
                }
            }
            _ => {
                // Iterate over every collection on collections
                // If collection has `Token::OBra(_)` or `Token::CBra(_)`,
                //  parse it? How?
                //  It is better to put this collection inside parse(&collection), but is it any good?
                // Return a collection.
                let mut parsed_collection = vec![];
                for collection in collections {
                    if collection.iter().any(|ti| {
                        matches!(
                            self.tokens.get(ti).unwrap(),
                            TokenKind::OpeningBracket | TokenKind::ClosingBracket
                        )
                    }) {
                        match self.reparse(&collection) {
                            Ok(n) => parsed_collection.push(n),
                            Err(e) => return Err(e),
                        }
                    } else {
                        parsed_collection.push(self.text(&collection)?);
                    }
                }
                Ok(Node::Collection {
                    items: parsed_collection,
                    #[cfg(test)]
                    start: pos.0,
                    #[cfg(test)]
                    end: pos.1,
                })
            }
        }
    }

    pub(crate) fn range(&self, fragment: &Vec<usize>) -> Result<Node, ParsingError> {
        if fragment.is_empty() {
            return Err(ParsingError::NoFragment);
        }
        enum State {
            First,
            Range,
            Second,
        }
        let mut start = true;
        let mut pos = (0_usize, 0_usize);
        let mut state = State::First;
        let mut limits = (String::new(), String::new());
        for token_index in fragment.iter() {
            if let Some(token) = self.tokens.get(token_index) {
                match token {
                    TokenKind::OpeningBracket => {
                        return Err(ParsingError::ExtraOpeningBracket(*token_index))
                    }
                    TokenKind::ClosingBracket => {
                        return Err(ParsingError::ExtraClosingBracket(*token_index))
                    }
                    TokenKind::Empty(_) | TokenKind::Comma => {
                        return Err(ParsingError::NoCommaInRange(*token_index))
                    }
                    // NOTE: potential a..z feature
                    TokenKind::Text(_) => return Err(ParsingError::NoTextInRange(*token_index)),
                    TokenKind::Number(l) => {
                        // below boilerplate code is for:
                        // in case, some stupid uses multiple number tokens one after another.
                        match state {
                            State::First => {
                                if start {
                                    pos.0 = *token_index;
                                    start = false;
                                }
                                limits.0.push_str(
                                    self.get_a_slice_of_cake(*token_index, *token_index + l)
                                        .as_str(),
                                );
                            }
                            State::Range => {
                                if start {
                                    return Err(ParsingError::StartLimitExpected(*token_index));
                                }
                                state = State::Second;
                                pos.1 = *token_index + 1;
                            }
                            State::Second => {
                                limits.1.push_str(
                                    self.get_a_slice_of_cake(*token_index, *token_index + l)
                                        .as_str(),
                                );
                            }
                        }
                    }
                    TokenKind::Range => match state {
                        State::First => {
                            state = State::Range;
                        }
                        State::Second | State::Range => {
                            return Err(ParsingError::ExtraRange(*token_index))
                        }
                    },
                }
            }
        }
        if limits.1.is_empty() {
            return Err(ParsingError::EndLimitExpected(pos.1));
        }
        Ok(Node::Range {
            from: limits.0,
            to: limits.1,
            #[cfg(test)]
            start: pos.0,
            #[cfg(test)]
            end: pos.1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_text() {
        let content = "Akşam vakti geldi!";
        let mut tokens = TokenMap::new();
        tokens.insert(0, TokenKind::Text(18));
        let fragment = vec![0_usize];
        let parser = Parser::new(content, tokens).unwrap();
        assert_eq!(
            Node::Text {
                content: "Akşam vakti geldi!".into(),
                start: 0,
                end: 18
            },
            parser.text(&fragment).unwrap()
        );
    }

    #[test]
    fn test_simple_text2() {
        let content = "Akşam";
        let mut tokens = TokenMap::new();
        tokens.insert(0, TokenKind::Text(5));
        let fragment = vec![0_usize];
        let parser = Parser::new(content, tokens).unwrap();
        assert_eq!(
            Node::Text {
                content: "Akşam".into(),
                start: 0,
                end: 5
            },
            parser.text(&fragment).unwrap()
        );
    }

    #[test]
    fn test_simple_text3() {
        let content = "A";
        let mut tokens = TokenMap::new();
        tokens.insert(0, TokenKind::Text(1));
        let fragment = vec![0_usize];
        let parser = Parser::new(content, tokens).unwrap();
        assert_eq!(
            Node::Text {
                content: "A".into(),
                start: 0,
                // as if sth. starts at 1st.
                end: 1
            },
            parser.text(&fragment).unwrap()
        );
    }
}
