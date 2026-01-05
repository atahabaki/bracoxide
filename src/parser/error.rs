#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    ExpectedString,
    ExpectedNumber,
}
