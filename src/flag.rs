pub struct Flag {
    escape_char: char,
}

impl Default for Flag {
    fn default() -> Self {
        Self { escape_char: '/' }
    }
}
