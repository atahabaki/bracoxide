#[derive(Debug)]
pub enum TokenizerError {
    NoData,
}

impl std::error::Error for TokenizerError {}
impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoData => write!(f, "Data is empty"),
        }
    }
}
