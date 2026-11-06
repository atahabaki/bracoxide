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
//! bracoxide = "0.1.2"
//! ```
//!
//! ```rust
//! use bracoxide::explode;
//!
//! fn main() {
//!     let content = "foo{1..3}bar";
//!     match explode(content) {
//!         Ok(expanded) => {
//!             // 1. `foo1bar`
//!             // 2. `foo2bar`
//!             // 3. `foo3bar`
//!             println!("Expanded patterns: {:?}", expanded);
//!         }
//!         Err(error) => {
//!             eprintln!("Error occurred: {:?}", error);
//!         }
//!     }
//! }
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
/// - `Interrupted`: The `should_stop` callback given to [expand_with] asked the expansion to stop.
///
/// Later releases may add variants, so a `match` on this enum outside the crate needs a `_` arm.
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum ExpansionError {
    /// Error indicating that a number conversion failed during expansion.
    NumConversionFailed(String),
    /// The `should_stop` callback given to [expand_with] returned `true`.
    Interrupted,
}

impl std::fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpansionError::NumConversionFailed(content) => {
                write!(f, "Number conversion of \"{}\" failed.", content)
            }
            ExpansionError::Interrupted => write!(f, "Expansion was interrupted."),
        }
    }
}

impl std::error::Error for ExpansionError {}

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
/// let node = Node::Text { message: "Hello".to_owned().into(), start: 0 };
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
    expand_with(node, &mut || false)
}

/// Same as [expand], but calls `should_stop` before making each string, the parts it joins
/// included, and stops with [ExpansionError::Interrupted] as soon as it returns `true`.
///
/// A short pattern can expand into more strings than fit in memory: `{1..4000000000}`
/// makes 4 billion. `expand_with` lets the caller stop such an expansion, when the user
/// presses Ctrl+C or a time limit passes, instead of waiting for it to finish.
///
/// # Arguments
///
/// * `node` - The parsed node to be expanded.
/// * `should_stop` - Called before each string is made, including the intermediate strings
///   that get joined into the output, so it can run more times than there are output strings.
///   Returning `true` stops the expansion.
///
/// # Returns
///
/// The same strings [expand] returns for `node`, if `should_stop` never returns `true`.
///
/// # Examples
///
/// ```
/// use bracoxide::{expand_with, parser::parse, tokenizer::tokenize, ExpansionError};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let node = parse(&tokenize("{1..4000000000}")?)?;
/// let mut made = 0;
/// let expanded = expand_with(&node, &mut || {
///     made += 1;
///     made > 3
/// });
/// assert_eq!(expanded, Err(ExpansionError::Interrupted));
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Returns [ExpansionError::Interrupted] when `should_stop` returns `true`, and the errors
/// of [expand] otherwise.
pub fn expand_with(
    node: &crate::parser::Node,
    should_stop: &mut dyn FnMut() -> bool,
) -> Result<Vec<String>, ExpansionError> {
    match node {
        parser::Node::Text { message, start: _ } => {
            stop_if(should_stop)?;
            Ok(vec![message.as_ref().to_owned()])
        }
        parser::Node::BraceExpansion {
            prefix,
            inside,
            postfix,
            start: _,
            end: _,
        } => {
            let mut inner = vec![];
            let prefixs: Vec<String> = if let Some(prefix) = prefix {
                expand_with(prefix, should_stop)?
            } else {
                vec!["".to_owned()]
            };
            let insides: Vec<String> = if let Some(inside) = inside {
                expand_with(inside, should_stop)?
            } else {
                vec!["".to_owned()]
            };
            let postfixs: Vec<String> = if let Some(postfix) = postfix {
                expand_with(postfix, should_stop)?
            } else {
                vec!["".to_owned()]
            };
            for prefix in &prefixs {
                for inside in &insides {
                    for postfix in &postfixs {
                        stop_if(should_stop)?;
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
                let expansions = expand_with(item, should_stop)?;
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
            // Get the numeric string length to be used later for zero padding
            let zero_pad = if from.starts_with('0') && from.len() > 1 || to.starts_with('0') {
                if from.len() >= to.len() {
                    from.len()
                } else {
                    to.len()
                }
            } else {
                0
            };
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
                stop_if(should_stop)?;
                inner.push(format!("{:0>width$}", i, width = zero_pad));
            }
            Ok(inner)
        }
    }
}

/// [ExpansionError::Interrupted] when `should_stop` returns `true`.
fn stop_if(should_stop: &mut dyn FnMut() -> bool) -> Result<(), ExpansionError> {
    if should_stop() {
        Err(ExpansionError::Interrupted)
    } else {
        Ok(())
    }
}

/// Same functionality as [bracoxidize] but with explosive materials. This crates' all
/// Error types (except the [OxidizationError]) implements [std::error::Error] trait. Why not get all the benefits from it?
pub fn explode(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let tokens = tokenizer::tokenize(content)?;
    let ast = parser::parse(&tokens)?;
    let expansions = expand(&ast)?;
    Ok(expansions)
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
    use std::sync::Arc;

    use super::parser::Node;
    use super::*;
    #[test]
    fn test_expand_complex() {
        assert_eq!(
            expand(&Node::BraceExpansion {
                prefix: Some(Box::new(Node::Text {
                    message: Arc::new("A".into()),
                    start: 0
                })),
                inside: Some(Box::new(Node::Collection {
                    items: vec![
                        Node::Text {
                            message: Arc::new("B".into()),
                            start: 2
                        },
                        Node::BraceExpansion {
                            prefix: Some(Box::new(Node::Text {
                                message: Arc::new("C".into()),
                                start: 4
                            })),
                            inside: Some(Box::new(Node::Collection {
                                items: vec![
                                    Node::Text {
                                        message: Arc::new("D".into()),
                                        start: 6
                                    },
                                    Node::Text {
                                        message: Arc::new("E".into()),
                                        start: 8
                                    },
                                ],
                                start: 5,
                                end: 9
                            })),
                            postfix: Some(Box::new(Node::Text {
                                message: Arc::new("F".into()),
                                start: 10
                            })),
                            start: 4,
                            end: 10,
                        },
                        Node::Text {
                            message: Arc::new("G".into()),
                            start: 12
                        }
                    ],
                    start: 1,
                    end: 13
                })),
                postfix: Some(Box::new(Node::BraceExpansion {
                    prefix: Some(Box::new(Node::Text {
                        message: Arc::new("H".into()),
                        start: 14
                    })),
                    inside: Some(Box::new(Node::Collection {
                        items: vec![
                            Node::Text {
                                message: Arc::new("J".into()),
                                start: 16
                            },
                            Node::Text {
                                message: Arc::new("K".into()),
                                start: 18
                            },
                        ],
                        start: 15,
                        end: 19
                    })),
                    postfix: Some(Box::new(Node::BraceExpansion {
                        prefix: Some(Box::new(Node::Text {
                            message: Arc::new("L".into()),
                            start: 20
                        })),
                        inside: Some(Box::new(Node::Range {
                            from: Arc::new("3".into()),
                            to: Arc::new("5".into()),
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
    #[test]
    fn test_expand_range_no_padding_bracoxidize() {
        assert_eq!(
            bracoxidize("A{1..10}"),
            Ok(vec![
                "A1".to_owned(),
                "A2".to_owned(),
                "A3".to_owned(),
                "A4".to_owned(),
                "A5".to_owned(),
                "A6".to_owned(),
                "A7".to_owned(),
                "A8".to_owned(),
                "A9".to_owned(),
                "A10".to_owned(),
            ])
        )
    }
    #[test]
    fn test_expand_range_no_padding_allowzero_bracoxidize() {
        assert_eq!(
            bracoxidize("A{0..10}"),
            Ok(vec![
                "A0".to_owned(),
                "A1".to_owned(),
                "A2".to_owned(),
                "A3".to_owned(),
                "A4".to_owned(),
                "A5".to_owned(),
                "A6".to_owned(),
                "A7".to_owned(),
                "A8".to_owned(),
                "A9".to_owned(),
                "A10".to_owned(),
            ])
        )
    }
    #[test]
    fn test_expand_range_zero_padding_bracoxidize_2() {
        assert_eq!(
            bracoxidize("A{00..10}"),
            Ok(vec![
                "A00".to_owned(),
                "A01".to_owned(),
                "A02".to_owned(),
                "A03".to_owned(),
                "A04".to_owned(),
                "A05".to_owned(),
                "A06".to_owned(),
                "A07".to_owned(),
                "A08".to_owned(),
                "A09".to_owned(),
                "A10".to_owned(),
            ])
        )
    }
    #[test]
    fn test_expand_range_zero_padding_bracoxidize() {
        assert_eq!(
            bracoxidize("A{4..06}{01..003}"),
            Ok(vec![
                "A04001".to_owned(),
                "A04002".to_owned(),
                "A04003".to_owned(),
                "A05001".to_owned(),
                "A05002".to_owned(),
                "A05003".to_owned(),
                "A06001".to_owned(),
                "A06002".to_owned(),
                "A06003".to_owned(),
            ])
        )
    }
    #[test]
    fn test_do_not_ignore_digits_before_any_backslashes() {
        assert_eq!(
            bracoxidize("1\\\\{a,b}"),
            Ok(vec!["1\\a".to_owned(), "1\\b".to_owned()])
        )
    }

    /// The parsed node of `content`, for the tests of [expand_with].
    fn parsed(content: &str) -> Node {
        let tokens = tokenizer::tokenize(content).expect("the pattern tokenizes");
        parser::parse(&tokens).expect("the pattern parses")
    }
    #[test]
    fn test_expand_with_never_stopping_is_expand() {
        let node = parsed("A{B,C{D,E}F,G}H{J,K}L{3..5}");
        assert_eq!(expand_with(&node, &mut || false), expand(&node));
    }
    #[test]
    fn test_expand_with_stops_before_text() {
        let node = Node::Text {
            message: Arc::new("A".into()),
            start: 0,
        };
        assert_eq!(
            expand_with(&node, &mut || true),
            Err(ExpansionError::Interrupted)
        );
    }
    #[test]
    fn test_expand_with_stops_a_range() {
        // A range on its own: the parser always puts one in a brace expansion, which
        // checks too.
        let node = Node::Range {
            from: Arc::new("1".into()),
            to: Arc::new("100000".into()),
            start: 0,
            end: 0,
        };
        let mut asked = 0;
        let expanded = expand_with(&node, &mut || {
            asked += 1;
            asked > 3
        });
        assert_eq!(expanded, Err(ExpansionError::Interrupted));
        assert_eq!(asked, 4);
    }
    #[test]
    fn test_expand_with_stops_while_joining() {
        // The two ranges take 3000 calls (the second is joined to an empty prefix once),
        // then joining them makes a million strings.
        let node = parsed("{1..1000}{1..1000}");
        let mut asked = 0;
        let expanded = expand_with(&node, &mut || {
            asked += 1;
            asked > 3500
        });
        assert_eq!(expanded, Err(ExpansionError::Interrupted));
        assert_eq!(asked, 3501);
    }
}
