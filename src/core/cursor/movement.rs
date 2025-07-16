use super::position::Position;

/// Cursor movement operations
pub struct Movement;

impl Movement {
    /// Move cursor up by one line
    pub fn up(pos: Position) -> Position {
        Position::new(pos.line.saturating_sub(1), pos.column)
    }
    
    /// Move cursor down by one line
    pub fn down(pos: Position) -> Position {
        Position::new(pos.line + 1, pos.column)
    }
    
    /// Move cursor left by one column
    pub fn left(pos: Position) -> Position {
        Position::new(pos.line, pos.column.saturating_sub(1))
    }
    
    /// Move cursor right by one column
    pub fn right(pos: Position) -> Position {
        Position::new(pos.line, pos.column + 1)
    }
}