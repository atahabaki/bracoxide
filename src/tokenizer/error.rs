#[derive(Debug)]
pub enum TokenizerError {
    NoData,
}

impl crate::Error for TokenizerError {}
impl crate::Display for TokenizerError {
    fn fmt(&self, f: &mut crate::Formatter<'_>) -> crate::FmtResult {
        match self {
            Self::NoData => write!(f, "Data is empty"),
        }
    }
}
