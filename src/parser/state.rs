use crate::Flag;

pub struct ParserState<'a> {
    content: &'a str,
    flag: Flag,
    position: usize,
}

impl<'a> Default for ParserState<'a> {
    fn default() -> Self {
        Self { content: Default::default(), flag: Default::default(), position: Default::default() }
    }
}

impl<'a> ParserState<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            ..Default::default()
        }
    }
    pub fn get_escape_char(&self) -> char {
        self.flag.escape_char
    }
    pub fn inc_position(&mut self) {
        self.position += 1;
    }
    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }
}