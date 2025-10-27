#[derive(Debug, PartialEq)]
pub(crate) enum TokenizerWarning {
    RedundantEscape { position: usize },
}

pub(crate) type TokenizerWarnings = Vec<TokenizerWarning>;
