use crate::Warning;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenizerWarning {
    RedundantEscape { position: usize },
}

pub type TokenizerWarnings = Vec<TokenizerWarning>;

impl Warning for TokenizerWarning {
    fn message(&self) -> String {
        match self {
            TokenizerWarning::RedundantEscape { position } => {
                format!("Redundant escape char used at {position}. position",)
            }
        }
    }
}
