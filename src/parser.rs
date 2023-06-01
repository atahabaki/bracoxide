//! Provides functions and types for parsing tokens into an abstract syntax tree (AST).
//!
//! ## Overview
//!
//! The parser module is responsible for transforming a sequence of tokens into a structured AST representation.
//! It takes the output of the tokenizer and performs the necessary parsing operations to generate the AST nodes.
//! The AST can then be used for further processing, interpretation, or code generation.

/// Represents a node in the parsed AST.
///
/// The `Node` enum captures different elements in the parsed abstract syntax tree (AST).
/// It includes variants for representing text, brace expansions, and ranges.
#[derive(Debug, PartialEq)]
pub enum Node {
    /// Represents a text node.
    /// It contains the text value and the starting position of the text.
    Text(String, usize),
    /// Represents a brace expansion node.
    /// It includes the prefix, inside, and outside nodes, along with the
    /// starting positions.
    BraceExpansion(
        Option<Vec<Node>>, // Prefix nodes
        Vec<Node>,         // Inside nodes
        Option<Vec<Node>>, // Postfix nodes
        usize,             // Starting position
    ),
    /// Represents a range node.
    /// It contains the starting and ending numbers of the range, along with the
    /// starting position.
    Range(String, String, usize),
}

/// Represents an error that can occur during parsing.
///
/// The `ParsingError` enum captures different error scenarios that can happen during parsing.
#[derive(Debug, PartialEq)]
pub enum ParsingError {
    /// Indicates that there are no tokens to parse.
    NoTokens,
    /// Expected closing bra, not fond... e.g. `{0..3` => Expected Syntax: `{0..3}`
    CBraExpected(usize),
    /// Expected Range Ending number... e.g. `{0..`
    RangeEndLimitExpected(usize),
    /// Indicates an unpredicted parsing error.
    Unpredicted,
}

use crate::tokenizer::Token;

/// Parses a sequence of tokens into an abstract syntax tree (AST).
///
/// The `parse` function takes a vector of tokens as input and performs the parsing operation.
/// It returns a result with the parsed AST nodes on success, or a specific error on failure.
///
/// # Arguments
///
/// * `tokens` - A vector of tokens to be parsed.
///
/// # Returns
///
/// * `Result<Vec<Node>, ParsingError>` - A result containing the parsed AST nodes or an error.
///
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParsingError> {
    if tokens.is_empty() {
        return Err(ParsingError::NoTokens);
    }
    let mut nodes = Vec::<Node>::new();
    let mut iter = tokens.iter();
    // counting `{` and `}`
    let mut count = (0, 0);
    while let Some(token) = iter.next() {
        match token {
            Token::OBra(i) => {
                let mut i_iter = iter.clone();
                if let Some(n_token) = i_iter.next() {
                    match n_token {
                        Token::OBra(_) => todo!(),
                        Token::CBra(_) => todo!(),
                        Token::Comma(_) => todo!(),
                        Token::Text(_, _) => todo!(),
                        // Range start String, Start IndeX
                        Token::Number(start, six) => {
                            if let Some(r_token) = i_iter.next() {
                                match r_token {
                                    Token::OBra(_) => todo!(),
                                    Token::CBra(_) => todo!(),
                                    Token::Comma(_) => todo!(),
                                    Token::Text(_, _) => todo!(),
                                    Token::Number(_, _) => todo!(),
                                    Token::Range(rix) => {
                                        if let Some(nn_token) = i_iter.next() {
                                            match nn_token {
                                                // Range end String, End IndeX
                                                Token::Number(end, eix) => {
                                                    if let Some(c_token) = i_iter.next() {
                                                        match c_token {
                                                            Token::CBra(_) => {
                                                                nodes.push(Node::Range(start.clone(), end.clone(), six.clone()));
                                                                iter = i_iter;
                                                            },
                                                        _ => return Err(ParsingError::CBraExpected(eix.clone())),
                                                        }
                                                    } else {
                                                        return Err(ParsingError::CBraExpected(eix.clone()));
                                                    }
                                                },
                                                _ => return Err(ParsingError::RangeEndLimitExpected(rix+1)),
                                            }
                                        } else {
                                            return Err(ParsingError::RangeEndLimitExpected(rix+1));
                                        }
                                    }
                                }
                            }
                        }
                        Token::Range(_) => todo!(),
                    }
                }
            }
            Token::Text(content, _) => {}
            _ => {}
        }
    }
    Ok(nodes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::Token;

    #[test]
    fn test_range_basic() {
        assert_eq!(
            parse(vec![
                Token::OBra(0),
                Token::Number("0".into(), 1),
                Token::Range(2),
            ]),
            Err(ParsingError::RangeEndLimitExpected(3))
        );
        assert_eq!(
            parse(vec![
                Token::OBra(0),
                Token::Number("0".into(), 1),
                Token::Range(2),
                Token::Number("3".into(), 4),
            ]),
            Err(ParsingError::CBraExpected(4))
        );
        assert_eq!(
            parse(vec![
                Token::OBra(0),
                Token::Number("0".into(), 1),
                Token::Range(2),
                Token::Number("3".into(), 4),
                Token::CBra(5)
            ]),
            Ok(vec![Node::Range("0".into(), "3".into(), 1)])
        );
    }
}
