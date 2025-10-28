use crate::Phase;

pub trait Warning {
    fn message(&self) -> String;
    fn phase(&self) -> Phase {
        Phase::Tokenizer
    }
}
