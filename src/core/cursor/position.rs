/// Cursor position in the text buffer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    /// Create a new position
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
    
    /// Create position at origin (0, 0)
    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}