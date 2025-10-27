use crate::{Warning, tokenizer::Tokens};

#[derive(Debug, PartialEq)]
pub(crate) struct Artifact<A> {
    pub(crate) artifact: A,
    pub(crate) warnings: Vec<Warning>,
}

impl<A> Artifact<A> {
    pub(crate) const fn new(artifact: A) -> Self {
        Self {
            artifact,
            warnings: vec![],
        }
    }
    pub(crate) const fn with_warnings(artifact: A, warnings: Vec<Warning>) -> Self {
        Self { artifact, warnings }
    }
    pub(crate) const fn has_any_warning(&self) -> bool {
        self.warnings.is_empty()
    }
}

impl Artifact<Tokens> {
    pub(crate) const fn has_any_token(&self) -> bool {
        self.artifact.is_empty()
    }
}
