use crate::{Warning, tokenizer::Tokens};

#[derive(Debug, PartialEq, Eq)]
pub struct Artifact<A> {
    pub artifact: A,
    pub warnings: Vec<Warning>,
}

impl<A> Artifact<A> {
    #[cfg(test)]
    pub const fn new(artifact: A) -> Self {
        Self {
            artifact,
            warnings: vec![],
        }
    }
    pub const fn with_warnings(artifact: A, warnings: Vec<Warning>) -> Self {
        Self { artifact, warnings }
    }
}

impl Artifact<Tokens> {
    pub const fn has_any_token(&self) -> bool {
        self.artifact.is_empty()
    }
}
