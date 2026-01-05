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
//! To start using the bracoxide, add it as a dependency:
//!
//! ```shell
//! cargo add bracoxide
//! ```
//!
//! ```
//! use bracoxide::explode;
//!
//! fn main() {
//!     let content = "foo{1..3}bar";
//!     match explode(content) {
//!         Ok(expanded) => {
//!             assert_eq!(expanded, ["foo1bar", "foo2bar", "foo3bar"]);
//!             println!("Expanded patterns: {:?}", expanded);
//!         }
//!         Err(error) => {
//!             eprintln!("Error occurred: {:?}", error);
//!         }
//!     }
//! }
//! ```
//!
//! We hope you find bracoxide to be a valuable tool in your Rust projects.
//! Happy string expansion!

pub(crate) mod parser;
pub(crate) use std::error::Error;

/// Put bash like brace expression and get all the possible outcomes
/// with proper error messages.
/// 
/// ```
/// use bracoxide::explode;
/// 
/// fn main() {
///     let content = "mkdir -p ~/{Desktop,{Mus,Publ}ic,{Do{cument,wnload},Template,Video,Picture}s}";
///     assert_eq!(explode(content).unwrap(), vec![
///         "mkdir -p ~/Desktop".to_string(),
///         "mkdir -p ~/Music".to_string(),
///         "mkdir -p ~/Public".to_string(),
///         "mkdir -p ~/Documents".to_string(),
///         "mkdir -p ~/Downloads".to_string(),
///         "mkdir -p ~/Templates".to_string(),
///         "mkdir -p ~/Videos".to_string(),
///         "mkdir -p ~/Pictures".to_string(),
///     ]);
/// }
/// ```
pub fn explode(content: &str) -> Result<Vec<String>, Box<dyn Error>> {
    todo!()
}

/// To customize the behaviour of the bracoxide explosion
/// From changing escape char to collecting warnings, what
/// may comes out of your head, we tried to do it somehow.
pub struct Flag {
    /// Used to escape special chars to escape
    /// For instance,
    /// ```
    /// use bracoxide::explode;
    ///
    /// fn main() {
    ///     let content = "J{ohn,ack} loves '%{'";
    ///     assert_eq!(
    ///         explode(content).unwrap(),
    ///         vec![
    ///             "John loves '{'".to_string(),
    ///             "Jack loves '{'".to_string()
    ///         ]);
    /// }
    /// ```
    pub escape_char: char,
    /// Some not intended use cases causes bracoxide to
    /// generate warnings such as "Incorrect use of escape
    /// char"... If that's what you don't want in your project
    /// you can safely disable. By default collects.
    pub collect_warnings: bool,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            collect_warnings: true,
            escape_char: '%',
        }
    }
}

pub fn explode_with_flags(
    content: &str,
    flag: Flag,
) -> Result<Vec<String>, Box<dyn Error>> {
    todo!()
}
