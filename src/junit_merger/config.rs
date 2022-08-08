#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub indent_char: u8,
    pub indent_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            indent_char: b' ',
            indent_size: 3,
        }
    }
}
