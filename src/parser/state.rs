pub struct ParserState {
    byte_idx: usize,
    // For end-user error message
    char_idx: usize,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl ParserState {
    pub fn new() -> Self {
        Self {
            byte_idx: 0,
            char_idx: 0,
        }
    }
    pub fn get_idx(&self) -> (usize, usize) {
        (self.byte_idx, self.char_idx)
    }
    pub fn get_byte_idx(&self) -> usize {
        self.byte_idx
    }
    pub fn get_char_idx(&self) -> usize {
        self.char_idx
    }
    pub fn set_byte_idx(&mut self, byte_idx: usize) {
        self.byte_idx = byte_idx;
    }
    pub fn set_char_idx(&mut self, char_idx: usize) {
        self.char_idx = char_idx;
    }
    pub fn set_idx(&mut self, byte_idx: usize, char_idx: usize) {
        self.byte_idx = byte_idx;
        self.char_idx = char_idx;
    }
    pub fn set_idx_tuple(&mut self, idx: (usize, usize)) {
        self.byte_idx = idx.0;
        self.char_idx = idx.1;
    }
    pub fn inc_byte_idx(&mut self) {
        self.byte_idx += 1;
    }
    pub fn inc_char_idx(&mut self) {
        self.char_idx += 1;
    }
    pub fn inc_idx_by(&mut self, amount_byte: usize, amount_char: usize) {
        self.byte_idx += amount_byte;
        self.char_idx += amount_char;
    }
    pub fn inc_idx_with_byte(&mut self, amount: usize) {
        self.byte_idx += amount;
        self.char_idx += 1;
    }
    pub fn inc_byte_idx_by(&mut self, amount: usize) {
        self.byte_idx += amount;
    }
    pub fn inc_char_idx_by(&mut self, amount: usize) {
        self.char_idx += amount;
    }
}
