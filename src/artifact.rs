use crate::tokenizer::{TokenizerWarning, Tokens};

#[derive(Debug, PartialEq, Eq)]
pub struct Artifact<A, W> {
    pub artifact: A,
    pub warnings: Vec<W>,
}

impl<A, W> Artifact<A, W> {
    #[cfg(test)]
    pub const fn new(artifact: A) -> Self {
        Self {
            artifact,
            warnings: vec![],
        }
    }
    pub const fn with_warnings(artifact: A, warnings: Vec<W>) -> Self {
        Self { artifact, warnings }
    }
}

impl Artifact<Tokens, TokenizerWarning> {
    pub const fn has_any_token(&self) -> bool {
        self.artifact.is_empty()
    }
}
