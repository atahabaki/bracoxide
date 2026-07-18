#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    Expected(String),
    ExpectedString,
    ExpectedNumber,
}
