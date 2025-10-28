use crate::Range;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) enum BufferState {
    Escape,
    #[default]
    Text,
    Number,
    TokenPushed,
}

#[derive(Debug)]
pub(super) struct TokenizerState {
    // Opening bracket and closing bracket count, respectively
    count: (usize, usize),
    // it is actually mix of current state & previous state.
    previous_buffer_state: Option<BufferState>,
    range: Range<usize>,
}

impl Default for TokenizerState {
    fn default() -> Self {
        Self {
            count: (0, 0), // It kinda  looks like an owl. Hi, owl>
            previous_buffer_state: None,
            range: Range { start: 0, end: 0 },
        }
    }
}

impl TokenizerState {
    pub const fn get_previous_buffer_state(&self) -> Option<BufferState> {
        self.previous_buffer_state
    }
    pub fn is_escape(&self) -> bool {
        self.previous_buffer_state == Some(BufferState::Escape)
    }
    pub const fn set_state_number(&mut self) {
        self.previous_buffer_state = Some(BufferState::Number);
    }
    pub const fn set_state_text(&mut self) {
        self.previous_buffer_state = Some(BufferState::Text);
    }
    pub const fn set_state_token(&mut self) {
        self.previous_buffer_state = Some(BufferState::TokenPushed);
    }
    pub const fn set_escape(&mut self, is_escape: bool) {
        let previous_buffer_state = self.previous_buffer_state;
        if is_escape {
            self.previous_buffer_state = Some(BufferState::Escape);
        } else {
            // I can not imagine any other rational stiuations than:
            // Feel free to create another example, that breaks this mindset?
            // "These are called '\{', '\}' {Opening,Closing} bracket"
            // See, I could escape a char inside curly braces but that will be a char no matter what, so the next state
            // certainly be text.
            self.previous_buffer_state = previous_buffer_state;
        }
    }
    pub const fn is_inside_curly_brackets(&self) -> bool {
        self.count.0 > self.count.1
    }
    pub const fn increment_obra(&mut self) {
        self.count.0 += 1;
    }
    pub const fn increment_cbra(&mut self) {
        self.count.1 += 1;
    }
    pub const fn increment_range_end(&mut self) {
        self.range.end += 1;
    }
    #[cfg(test)]
    pub const fn decrement_range_end(&mut self) {
        self.range.end -= 1;
    }
    pub const fn new_range(&mut self, start: usize) {
        self.range = Range {
            start,
            end: start + 1,
        }
    }
    pub fn get_range(&self) -> Range<usize> {
        self.range.clone()
    }
    pub const fn set_range_end_if_biggers_than(&mut self, this: usize) {
        if self.range.end > this {
            self.range.end = this;
        }
    }
}
