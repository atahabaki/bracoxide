pub struct Flag {
    pub escape_char: char,
    pub supress_warning: bool,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            escape_char: '%',
            supress_warning: false,
        }
    }
}
