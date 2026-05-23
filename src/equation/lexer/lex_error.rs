#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexError {
    Eof,
    InvalidToken(u32),
}
