/*
 * This file is part of bracoxide.
 *
 * bracoxide is under MIT license.
 *
 * Copyright (c) 2023 A. Taha Baki <atahabaki@pm.me>
 */

//! This crate provides a powerful and intuitive way to perform string brace expansion.
//! Brace expansion is a feature commonly found in shells and text processing tools,
//! allowing you to generate all possible combinations of strings specified within
//! curly braces.
//!
//! ## Features
//! - **Simple and Easy-to-Use**: With the bracoxide crate, expanding brace patterns in
//! strings becomes a breeze. Just pass in your input string, and the crate will
//! generate all possible combinations for you.
//!
//! - **Flexible Brace Expansion**: The crate supports various brace expansion patterns,
//! including numeric ranges ({0..9}), comma-separated options ({red,green,blue}),
//! nested expansions ({a{b,c}d}, {x{1..3},y{4..6}}), and more.
//!
//! - **Robust Error Handling**: The crate provides detailed error handling, allowing you
//! to catch and handle any issues that may arise during the tokenization and expansion
//! process.
//!
//! - **Lightweight and Fast**: Designed to be efficient and performant, ensuring quick
//! and reliable string expansion operations.
//!
//! ## Getting Started
//!
//! To start using the bracoxide crate, add it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bracoxide = "0.1.0"
//! ```
//!
//! We hope you find the str expand crate to be a valuable tool in your Rust projects.
//! Happy string expansion!

pub mod parser;
pub mod tokenizer;

/// An error type representing the failure to expand a parsed node.
///
/// This enum is used to indicate errors that can occur during the expansion of a parsed node.
/// It provides detailed information about the specific type of error encountered.
///
/// # Variants
///
/// - `NumConversionFailed(String)`: An error indicating that a number conversion failed during expansion.
///                                 It contains a string representing the value that failed to be converted.
#[derive(Debug, PartialEq)]
pub enum ExpansionError {
    /// Error indicating that a number conversion failed during expansion.
    NumConversionFailed(String),
}

/// Expands the given parsed node into a vector of strings representing the expanded values.
///
/// # Arguments
///
/// * `node` - The parsed node to be expanded.
///
/// # Returns
///
/// Returns a result containing a vector of strings representing the expanded values. If the
/// expansion fails, an `ExpansionError` is returned.
///
/// # Examples
///
/// ```
/// use bracoxide::parser::Node;
/// use bracoxide::{expand, ExpansionError};
///
/// let node = Node::Text { message: "Hello".to_owned(), start: 0 };
/// let expanded = expand(&node);
/// assert_eq!(expanded, Ok(vec!["Hello".to_owned()]));
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// Returns an `ExpansionError` if the expansion fails due to various reasons, such as
/// failed number conversion or invalid syntax.
///
/// # Safety
///
/// This function operates on valid parsed nodes and does not use unsafe code internally.
pub fn expand(node: &crate::parser::Node) -> Result<Vec<String>, ExpansionError> {
    match node {
        parser::Node::Text { message, start: _ } => Ok(vec![message.to_owned()]),
        parser::Node::BraceExpansion {
            prefix,
            inside,
            postfix,
            start: _,
            end: _,
        } => {
            let mut inner = vec![];
            let prefixs: Vec<String> = if let Some(prefix) = prefix {
                expand(prefix)?
            } else {
                vec!["".to_owned()]
            };
            let insides: Vec<String> = if let Some(inside) = inside {
                expand(inside)?
            } else {
                vec!["".to_owned()]
            };
            let postfixs: Vec<String> = if let Some(postfix) = postfix {
                expand(postfix)?
            } else {
                vec!["".to_owned()]
            };
            for prefix in &prefixs {
                for inside in &insides {
                    for postfix in &postfixs {
                        inner.push(format!("{}{}{}", prefix, inside, postfix));
                    }
                }
            }
            Ok(inner)
        }
        parser::Node::Collection {
            items,
            start: _,
            end: _,
        } => {
            let mut inner = vec![];
            for item in items {
                let expansions = expand(item)?;
                inner.extend(expansions);
            }
            Ok(inner)
        }
        parser::Node::Range {
            from,
            to,
            start: _,
            end: _,
        } => {
            let from = if let Ok(from) = from.parse::<usize>() {
                from
            } else {
                return Err(ExpansionError::NumConversionFailed(from.to_string()));
            };

            let to = if let Ok(to) = to.parse::<usize>() {
                to
            } else {
                return Err(ExpansionError::NumConversionFailed(to.to_string()));
            };
            let range = from..=to;
            let mut inner = vec![];
            for i in range {
                inner.push(i.to_string());
            }
            Ok(inner)
        }
    }
}

/// Errors that can occur during the Brace Expansion process.
#[derive(Debug, PartialEq)]
pub enum OxidizationError {
    TokenizationError(tokenizer::TokenizationError),
    ParsingError(parser::ParsingError),
    ExpansionError(ExpansionError),
}

/// Bracoxidize the provided content by tokenizing, parsing, and expanding brace patterns.
///
/// # Arguments
///
/// * `content` - The input string to be processed.
///
/// # Returns
///
/// Returns a `Result` containing the expanded brace patterns as `Vec<String>`,
/// or an `OxidizationError` if an error occurs during the process.
///
/// # Examples
///
/// ```rust
/// use bracoxide::{bracoxidize, OxidizationError};
///
/// fn main() {
///     let content = "foo{1..3}bar";
///     match bracoxidize(content) {
///         Ok(expanded) => {
///             println!("Expanded patterns: {:?}", expanded);
///         }
///         Err(error) => {
///             eprintln!("Error occurred: {:?}", error);
///         }
///     }
/// }
/// ```
pub fn bracoxidize(content: &str) -> Result<Vec<String>, OxidizationError> {
    // Tokenize the input string
    let tokens = match tokenizer::tokenize(content) {
        Ok(tokens) => tokens,
        Err(error) => return Err(OxidizationError::TokenizationError(error)),
    };

    // Parse the tokens into an abstract syntax tree
    let ast = match parser::parse(&tokens) {
        Ok(ast) => ast,
        Err(error) => return Err(OxidizationError::ParsingError(error)),
    };

    // Expand the brace patterns in the AST
    let expanded = match expand(&ast) {
        Ok(expanded) => expanded,
        Err(error) => return Err(OxidizationError::ExpansionError(error)),
    };

    Ok(expanded)
}

#[cfg(test)]
mod tests {
    use super::parser::Node;
    use super::*;
    #[test]
    fn test_expand_complex() {
        assert_eq!(
            expand(&Node::BraceExpansion {
                prefix: Some(Box::new(Node::Text {
                    message: "A".into(),
                    start: 0
                })),
                inside: Some(Box::new(Node::Collection {
                    items: vec![
                        Node::Text {
                            message: "B".into(),
                            start: 2
                        },
                        Node::BraceExpansion {
                            prefix: Some(Box::new(Node::Text {
                                message: "C".into(),
                                start: 4
                            })),
                            inside: Some(Box::new(Node::Collection {
                                items: vec![
                                    Node::Text {
                                        message: "D".into(),
                                        start: 6
                                    },
                                    Node::Text {
                                        message: "E".into(),
                                        start: 8
                                    },
                                ],
                                start: 5,
                                end: 9
                            })),
                            postfix: Some(Box::new(Node::Text {
                                message: "F".into(),
                                start: 10
                            })),
                            start: 4,
                            end: 10,
                        },
                        Node::Text {
                            message: "G".into(),
                            start: 12
                        }
                    ],
                    start: 1,
                    end: 13
                })),
                postfix: Some(Box::new(Node::BraceExpansion {
                    prefix: Some(Box::new(Node::Text {
                        message: "H".into(),
                        start: 14
                    })),
                    inside: Some(Box::new(Node::Collection {
                        items: vec![
                            Node::Text {
                                message: "J".into(),
                                start: 16
                            },
                            Node::Text {
                                message: "K".into(),
                                start: 18
                            },
                        ],
                        start: 15,
                        end: 19
                    })),
                    postfix: Some(Box::new(Node::BraceExpansion {
                        prefix: Some(Box::new(Node::Text {
                            message: "L".into(),
                            start: 20
                        })),
                        inside: Some(Box::new(Node::Range {
                            from: "3".into(),
                            to: "5".into(),
                            start: 21,
                            end: 26
                        })),
                        postfix: None,
                        start: 20,
                        end: 26
                    })),
                    start: 14,
                    end: 26
                })),
                start: 0,
                end: 26
            }),
            Ok(vec![
                "ABHJL3".to_owned(),
                "ABHJL4".to_owned(),
                "ABHJL5".to_owned(),
                "ABHKL3".to_owned(),
                "ABHKL4".to_owned(),
                "ABHKL5".to_owned(),
                "ACDFHJL3".to_owned(),
                "ACDFHJL4".to_owned(),
                "ACDFHJL5".to_owned(),
                "ACDFHKL3".to_owned(),
                "ACDFHKL4".to_owned(),
                "ACDFHKL5".to_owned(),
                "ACEFHJL3".to_owned(),
                "ACEFHJL4".to_owned(),
                "ACEFHJL5".to_owned(),
                "ACEFHKL3".to_owned(),
                "ACEFHKL4".to_owned(),
                "ACEFHKL5".to_owned(),
                "AGHJL3".to_owned(),
                "AGHJL4".to_owned(),
                "AGHJL5".to_owned(),
                "AGHKL3".to_owned(),
                "AGHKL4".to_owned(),
                "AGHKL5".to_owned(),
            ])
        )
    }
    #[test]
    fn test_expand_complex_bracoxidize() {
        assert_eq!(
            bracoxidize("A{B,C{D,E}F,G}H{J,K}L{3..5}"),
            Ok(vec![
                "ABHJL3".to_owned(),
                "ABHJL4".to_owned(),
                "ABHJL5".to_owned(),
                "ABHKL3".to_owned(),
                "ABHKL4".to_owned(),
                "ABHKL5".to_owned(),
                "ACDFHJL3".to_owned(),
                "ACDFHJL4".to_owned(),
                "ACDFHJL5".to_owned(),
                "ACDFHKL3".to_owned(),
                "ACDFHKL4".to_owned(),
                "ACDFHKL5".to_owned(),
                "ACEFHJL3".to_owned(),
                "ACEFHJL4".to_owned(),
                "ACEFHJL5".to_owned(),
                "ACEFHKL3".to_owned(),
                "ACEFHKL4".to_owned(),
                "ACEFHKL5".to_owned(),
                "AGHJL3".to_owned(),
                "AGHJL4".to_owned(),
                "AGHJL5".to_owned(),
                "AGHKL3".to_owned(),
                "AGHKL4".to_owned(),
                "AGHKL5".to_owned(),
            ])
        )
    }
}
