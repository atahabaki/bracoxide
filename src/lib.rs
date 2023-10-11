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
//! We hope you find the str expand crate to be a valuable tool in your Rust projects.
//! Happy string expansion!

pub(crate) mod parser;
pub(crate) mod tokenizer;

use parser::{Parser, ParsingError};
use tokenizer::{TokenizationError, Tokenizer};

/// An error type representing the failure to expand a parsed node.
///
/// This enum is used to indicate errors that can occur during the expansion of a parsed node.
/// It provides detailed information about the specific type of error encountered.
///
/// # Variants
///
/// - `NumConversionFailed(String)`: An error indicating that a number conversion failed during expansion.
///                                 It contains a string representing the value that failed to be converted.
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[cfg_attr(feature = "simplerr", derive(Debug))]
pub enum ExpansionError {
    /// Error indicating that a number conversion failed during expansion.
    NumConversionFailed(String),
}

impl std::fmt::Display for ExpansionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpansionError::NumConversionFailed(content) => {
                write!(f, "Number conversion of \"{}\" failed.", content)
            }
        }
    }
}

#[cfg(feature = "simplerr")]
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
pub(crate) fn expand(node: &crate::parser::Node) -> Result<Vec<String>, ExpansionError> {
    match node {
        parser::Node::Text {
            content,
            #[cfg(test)]
                start: _,
            #[cfg(test)]
                end: _,
        } => Ok(vec![content.to_owned()]),
        parser::Node::BraceExpansion {
            prefix,
            inside,
            postfix,
            #[cfg(test)]
                start: _,
            #[cfg(test)]
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
            #[cfg(test)]
                start: _,
            #[cfg(test)]
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
            #[cfg(test)]
                start: _,
            #[cfg(test)]
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

/// Same functionality as [bracoxidize] but with explosive materials. This crates' all
/// Error types (except the [OxidizationError]) implements [std::error::Error] trait. Why not get all the benefits from it?
#[cfg(feature = "simplerr")]
pub fn explode(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(content)?;
    tokenizer.tokenize()?;
    let parser = Parser::from_tokenizer(tokenizer)?;
    let ast = parser.parse()?;
    let expansions = expand(&ast)?;
    Ok(expansions)
}

/// Errors that can occur during the Brace Expansion process.
#[derive(PartialEq)]
#[cfg_attr(test, derive(Debug))]
#[cfg_attr(feature = "simplerr", derive(Debug))]
pub enum OxidizationError {
    TokenizerError(TokenizationError),
    ParserError(ParsingError),
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
pub fn bracoxidize(content: impl ToString) -> Result<Vec<String>, OxidizationError> {
    let content = content.to_string();
    match Tokenizer::new(&content) {
        Ok(mut tokenizer) => match tokenizer.tokenize() {
            Ok(_) => match Parser::from_tokenizer(tokenizer) {
                Ok(parser) => match parser.parse() {
                    Ok(n) => match expand(&n) {
                        Ok(res) => Ok(res),
                        Err(e) => Err(OxidizationError::ExpansionError(e)),
                    },
                    Err(e) => Err(OxidizationError::ParserError(e)),
                },
                Err(e) => Err(OxidizationError::ParserError(e)),
            },
            Err(e) => Err(OxidizationError::TokenizerError(e)),
        },
        Err(e) => Err(OxidizationError::TokenizerError(e)),
    }
}
