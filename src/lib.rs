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

/// Defines the possible types of tokens that can be encountered during the process of
/// tokenization.
///
/// The [Token] enum is used to represent different types of tokens that can be produced
/// while tokenizing a string. Each variant of the [Token] enum corresponds to a specific
/// type of token, such as opening brace, closing brace, comma, text, number, or range
/// operator.
///
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Represents an opening brace `{`.
    OBra,
    /// Represents a closing brace `}`.
    CBra,
    /// Represents a comma `,`.
    Comma,
    /// Represents any non-number text.
    ///
    /// The associated `String` contains the text value.
    Text(String),
    /// Represents a number.
    ///
    /// The associated `String` contains the numeric value.
    Number(String),
    /// Represents the range operator `..`.
    Range,
}

/// Represents the possible errors that can occur during the tokenization.
///
/// # Example
///
/// ```rust,no_run
/// use bracoxide::TokenizationError;
///
/// let content = "{a, b, c, d";
/// let tokenization_result = bracoxide::tokenize(content);
/// assert_eq!(tokenization_result, Err(TokenizationError::FormatNotSupported));
/// ```
#[derive(Debug, PartialEq)]
pub enum TokenizationError {
    /// The content to be tokenized is empty.
    EmptyContent,
    /// The input content has an unsupported format (e.g., only an opening brace or closing
    /// brace).
    FormatNotSupported,
    /// The opening and closing braces in the input content do not match.
    BraceMismatch,
    /// The input content does not contain any braces.
    NoBraces,
    /// An unexpected error occurred during tokenization.
    ///
    /// This error indicates a situation that is considered highly unlikely or
    /// impossible to occur during normal operation. If you encounter this
    /// error, please report it to the library maintainers for further investigation.
    Unpredicted,
}

/// Tokenizes the provided content string and produces a vector of tokens.
///
/// This function is part of the `bracoxide` crate and is used to tokenize a given string `content`.
/// The tokenization process splits the string into meaningful units called tokens, which can be
/// further processed or analyzed as needed.
///
/// # Arguments
///
/// * `content` - The string to be tokenized.
///
/// # Returns
///
/// * `Result<Vec<Token>, TokenizationError>` - A result that contains a vector of tokens if the tokenization
///   is successful, or a [TokenizationError] if an error occurs during the tokenization process.
///
/// # Errors
///
/// The function can return the following errors:
///
/// * [TokenizationError::EmptyContent] - If the `content` string is empty.
/// * [TokenizationError::NoBraces] - If the `content` string does not contain any braces.
/// * [TokenizationError::FormatNotSupported] - If the `content` string has an unsupported format, such as
///   only an opening brace or closing brace without a corresponding pair.
/// * [TokenizationError::BraceMismatch] - If the opening and closing braces in the `content` string do not match.
/// * [TokenizationError::Unpredicted] - An unexpected error occurred during the tokenization process. This error
///   indicates a situation that is considered highly unlikely or impossible to occur during normal operation.
///   If you encounter this error, please report it to the maintainers of the `bracoxide` crate for further investigation.
///
/// # Examples
///
/// ```
/// use bracoxide::{Token, TokenizationError, tokenize};
///
/// let content = "{1, 2, 3}";
/// let tokens = tokenize(content);
///
/// match tokens {
///     Ok(tokens) => {
///         println!("Tokenization successful!");
///         for token in tokens {
///             println!("{:?}", token);
///         }
///     }
///     Err(error) => {
///         eprintln!("Tokenization failed: {:?}", error);
///     }
/// }
/// ```
///
/// In this example, the `tokenize` function from the `bracoxide` crate is used to tokenize the content string "{1, 2, 3}".
/// If the tokenization is successful, the resulting tokens are printed. Otherwise, the corresponding error is displayed.
pub fn tokenize(content: &str) -> Result<Vec<Token>, TokenizationError> {
    if content.is_empty() {
        return Err(TokenizationError::EmptyContent);
    }
    let mut tokens = Vec::<Token>::new();
    let mut is_escape = false;
    // opening, closing
    let mut count = (0, 0);
    // text_buffer, number_buffer
    let mut buffers = (String::new(), String::new());
    let mut iter = content.chars();
    // Push buffers into tokens.
    let tokenize_buffers = |tokens: &mut Vec<Token>, buffers: &mut (String, String)| {
        if !buffers.0.is_empty() {
            tokens.push(Token::Text(buffers.0.clone()));
            buffers.0.clear();
        }
        if !buffers.1.is_empty() {
            tokens.push(Token::Number(buffers.1.clone()));
            buffers.1.clear();
        }
    };
    while let Some(c) = iter.next() {
        println!("{:?}, {}", buffers, c);
        match (c, is_escape) {
            (_, true) => {
                buffers.0.push(c);
                buffers.1.clear();
                is_escape = false;
            }
            ('\\', false) => is_escape = true,
            // @1: COMMENT
            // Look it is '{' OR '}' OR ','
            // No other c value can pass this match ARM
            // And now look to @2
            ('{' | '}' | ',', _) => {
                tokenize_buffers(&mut tokens, &mut buffers);
                match c {
                    '{' => {
                        count.0 += 1;
                        tokens.push(Token::OBra);
                    }
                    '}' => {
                        count.1 += 1;
                        tokens.push(Token::CBra);
                    }
                    ',' => tokens.push(Token::Comma),
                    // @2: COMMENT
                    // Look @1 the above catch, you see
                    // c can be just '{' OR '}' OR ','.
                    // AND Why the god damn rust wants me to handle all cases,
                    // Where I got covered all cases above.
                    _ => return Err(TokenizationError::Unpredicted),
                }
            }
            ('.', _) => {
                let mut r_iter = iter.clone();
                if let Some(cx) = r_iter.next() {
                    match cx {
                        '.' => {
                            tokenize_buffers(&mut tokens, &mut buffers);
                            tokens.push(Token::Range);
                            iter = r_iter;
                            continue;
                        }
                        _ => buffers.0.push(c),
                    }
                } else {
                    buffers.0.push(c);
                }
            }
            ('0'..='9', _) => {
                if !buffers.0.is_empty() {
                    tokens.push(Token::Text(buffers.0.clone()));
                    buffers.0.clear();
                }
                buffers.1.push(c);
            }
            _ => {
                if !buffers.1.is_empty() {
                    tokens.push(Token::Number(buffers.1.clone()));
                    buffers.1.clear();
                }
                buffers.0.push(c);
            }
        }
    }
    match count {
        (0, 0) => return Err(TokenizationError::NoBraces),
        (0, _) | (_, 0) => return Err(TokenizationError::FormatNotSupported),
        (_, _) => {
            if count.0 != count.1 {
                return Err(TokenizationError::BraceMismatch);
            }
        }
    }
    tokenize_buffers(&mut tokens, &mut buffers);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_content() {
        assert_eq!(tokenize(""), Err(TokenizationError::EmptyContent));
        assert_eq!(
            tokenize(String::new().as_str()),
            Err(TokenizationError::EmptyContent)
        );
    }

    #[test]
    fn test_no_braces() {
        assert_eq!(tokenize("a"), Err(TokenizationError::NoBraces));
        assert_eq!(tokenize("1..3"), Err(TokenizationError::NoBraces));
        assert_eq!(tokenize("a,b"), Err(TokenizationError::NoBraces));
        assert_eq!(
            tokenize("arst1..3.(arst)xt"),
            Err(TokenizationError::NoBraces)
        );
    }

    #[test]
    fn test_format_not_supported() {
        assert_eq!(
            tokenize("{a, b, c, d"),
            Err(TokenizationError::FormatNotSupported)
        );
        assert_eq!(
            tokenize("{{a, b, c, d"),
            Err(TokenizationError::FormatNotSupported)
        );
        assert_eq!(
            tokenize("a, b, c, d}}"),
            Err(TokenizationError::FormatNotSupported)
        );
        assert_eq!(
            tokenize("a{, b{, c{, d{"),
            Err(TokenizationError::FormatNotSupported)
        );
    }

    #[test]
    fn test_brace_mismatch() {
        assert_eq!(
            tokenize("a{b{c,de}f"),
            Err(TokenizationError::BraceMismatch)
        );
        assert_eq!(tokenize("a{..}}"), Err(TokenizationError::BraceMismatch));
        assert_eq!(
            tokenize("{a..3{a,b}}}"),
            Err(TokenizationError::BraceMismatch)
        );
        assert_eq!(tokenize("{}{}}"), Err(TokenizationError::BraceMismatch));
    }

    #[test]
    fn test_tokenize() {
        // Test case 1: {1..3}
        assert_eq!(
            tokenize("{1..3}"),
            Ok(vec![
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra
            ])
        );

        // Test case 2: {a,b,c}
        assert_eq!(
            tokenize("{a,b,c}"),
            Ok(vec![
                Token::OBra,
                Token::Text("a".to_owned()),
                Token::Comma,
                Token::Text("b".to_owned()),
                Token::Comma,
                Token::Text("c".to_owned()),
                Token::CBra
            ])
        );

        // Test case 3: {A{1,2},B{1,2}}
        assert_eq!(
            tokenize("{A{1,2},B{1,2}}"),
            Ok(vec![
                Token::OBra,
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::CBra
            ])
        );

        // Test case 4: A{1,2}B{1,2}
        assert_eq!(
            tokenize("A{1,2}B{1,2}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Comma,
                Token::Number("2".to_owned()),
                Token::CBra
            ])
        );

        // Test case 5: a{b{c,d}e{f,g}h}i
        assert_eq!(
            tokenize("a{b{c,d}e{f,g}h}i"),
            Ok(vec![
                Token::Text("a".to_owned()),
                Token::OBra,
                Token::Text("b".to_owned()),
                Token::OBra,
                Token::Text("c".to_owned()),
                Token::Comma,
                Token::Text("d".to_owned()),
                Token::CBra,
                Token::Text("e".to_owned()),
                Token::OBra,
                Token::Text("f".to_owned()),
                Token::Comma,
                Token::Text("g".to_owned()),
                Token::CBra,
                Token::Text("h".to_owned()),
                Token::CBra,
                Token::Text("i".to_owned())
            ])
        );

        // Test case 6: a{b{c,d},e{f,g}}h
        assert_eq!(
            tokenize("a{b{c,d},e{f,g}}h"),
            Ok(vec![
                Token::Text("a".to_owned()),
                Token::OBra,
                Token::Text("b".to_owned()),
                Token::OBra,
                Token::Text("c".to_owned()),
                Token::Comma,
                Token::Text("d".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("e".to_owned()),
                Token::OBra,
                Token::Text("f".to_owned()),
                Token::Comma,
                Token::Text("g".to_owned()),
                Token::CBra,
                Token::CBra,
                Token::Text("h".to_owned())
            ])
        );

        // Test case 7: A{1..3}B{2,5}
        assert_eq!(
            tokenize("A{1..3}B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 8: A{1..3},B{2,5}
        assert_eq!(
            tokenize("A{1..3},B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Comma,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 9: A{1..3}\,B{2,5}
        assert_eq!(
            tokenize("A{1..3}\\,B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text(",B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 10: A{1..3}.B{2,5}
        assert_eq!(
            tokenize("A{1..3}.B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Text(".B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );
        // Test case 11: A1..3.B{2,5}
        assert_eq!(
            tokenize("A1..3.B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::Text(".B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );

        // Test case 12: A{1..3}..B{2,5}
        assert_eq!(
            tokenize("A{1..3}..B{2,5}"),
            Ok(vec![
                Token::Text("A".to_owned()),
                Token::OBra,
                Token::Number("1".to_owned()),
                Token::Range,
                Token::Number("3".to_owned()),
                Token::CBra,
                Token::Range,
                Token::Text("B".to_owned()),
                Token::OBra,
                Token::Number("2".to_owned()),
                Token::Comma,
                Token::Number("5".to_owned()),
                Token::CBra
            ])
        );
    }
}
