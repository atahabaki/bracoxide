#[derive(Debug, PartialEq, Eq)]
pub enum TokenizerWarning {
    RedundantEscape { position: usize },
}

pub type TokenizerWarnings = Vec<TokenizerWarning>;
