use crate::{Result, RuskError};
use crate::plugins::implementations::language::syntax::SupportedLanguage;
use std::fs;
use std::path::Path;

/// Text buffer data structure with full editing capabilities
#[derive(Debug, Clone)]
pub struct Buffer {
    lines: Vec<String>,
    file_path: Option<String>,
    language: SupportedLanguage,
    modified: bool,
    cursor_line: usize,
    cursor_col: usize,
    selection_start: Option<(usize, usize)>,
    selection_end: Option<(usize, usize)>,
}

impl Buffer {
    /// Create a new empty buffer
    pub fn new() -> Self {
        Self {
            lines: vec![String::new()],
            file_path: None,
            language: SupportedLanguage::Text,
            modified: false,
            cursor_line: 0,
            cursor_col: 0,
            selection_start: None,
            selection_end: None,
        }
    }
    
    /// Create a buffer from file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        // Detect language from file extension
        let language = if let Some(extension) = path.as_ref().extension() {
            SupportedLanguage::from_extension(&extension.to_string_lossy())
        } else {
            SupportedLanguage::Text
        };
        
        if path.as_ref().exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| RuskError::io_string(format!("Failed to read file {}: {}", path_str, e)))?;
            
            let lines: Vec<String> = if content.is_empty() {
                vec![String::new()]
            } else {
                content.lines().map(|s| s.to_string()).collect()
            };
            
            Ok(Self {
                lines,
                file_path: Some(path_str),
                language,
                modified: false,
                cursor_line: 0,
                cursor_col: 0,
                selection_start: None,
                selection_end: None,
            })
        } else {
            // Create new file buffer
            Ok(Self {
                lines: vec![String::new()],
                file_path: Some(path_str),
                language,
                modified: true,
                cursor_line: 0,
                cursor_col: 0,
                selection_start: None,
                selection_end: None,
            })
        }
    }
    
    /// Save buffer to file
    pub fn save(&mut self) -> Result<()> {
        let path = self.file_path.as_ref()
            .ok_or_else(|| RuskError::Buffer("No file path set for buffer".to_string()))?;
        
        let content = self.lines.join("\n");
        fs::write(path, content)
            .map_err(|e| RuskError::io_string(format!("Failed to save file {}: {}", path, e)))?;
        
        self.modified = false;
        Ok(())
    }
    
    /// Save buffer to specific path
    pub fn save_as<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let content = self.lines.join("\n");
        
        fs::write(&path, content)
            .map_err(|e| RuskError::io_string(format!("Failed to save file {}: {}", path_str, e)))?;
        
        self.file_path = Some(path_str);
        self.modified = false;
        Ok(())
    }
    
    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// Check if buffer is modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }
    
    /// Get file path
    pub fn file_path(&self) -> Option<&str> {
        self.file_path.as_deref()
    }
    
    /// Get buffer language
    pub fn language(&self) -> &SupportedLanguage {
        &self.language
    }
    
    /// Get cursor position
    pub fn cursor_position(&self) -> (usize, usize) {
        (self.cursor_line, self.cursor_col)
    }
    
    /// Set cursor position
    pub fn set_cursor_position(&mut self, line: usize, col: usize) {
        self.cursor_line = line.min(self.lines.len().saturating_sub(1));
        self.cursor_col = col.min(self.lines[self.cursor_line].len());
    }
    
    /// Get line at index
    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.lines.get(index).map(|s| s.as_str())
    }
    
    /// Get all lines
    pub fn lines(&self) -> &[String] {
        &self.lines
    }
    
    /// Insert character at cursor position
    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        
        self.lines[self.cursor_line].insert(self.cursor_col, ch);
        self.cursor_col += 1;
        self.modified = true;
    }
    
    /// Insert string at cursor position
    pub fn insert_str(&mut self, s: &str) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        
        self.lines[self.cursor_line].insert_str(self.cursor_col, s);
        self.cursor_col += s.len();
        self.modified = true;
    }
    
    /// Delete character at cursor position
    pub fn delete_char(&mut self) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        
        if self.cursor_col < self.lines[self.cursor_line].len() {
            self.lines[self.cursor_line].remove(self.cursor_col);
            self.modified = true;
        } else if self.cursor_line + 1 < self.lines.len() {
            // Join with next line
            let next_line = self.lines.remove(self.cursor_line + 1);
            self.lines[self.cursor_line].push_str(&next_line);
            self.modified = true;
        }
    }
    
    /// Backspace at cursor position
    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
            self.lines[self.cursor_line].remove(self.cursor_col);
            self.modified = true;
        } else if self.cursor_line > 0 {
            // Join with previous line
            let current_line = self.lines.remove(self.cursor_line);
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].len();
            self.lines[self.cursor_line].push_str(&current_line);
            self.modified = true;
        }
    }
    
    /// Insert new line at cursor position
    pub fn insert_newline(&mut self) {
        if self.cursor_line >= self.lines.len() {
            return;
        }
        
        let current_line = self.lines[self.cursor_line].clone();
        let (before, after) = current_line.split_at(self.cursor_col);
        
        self.lines[self.cursor_line] = before.to_string();
        self.lines.insert(self.cursor_line + 1, after.to_string());
        
        self.cursor_line += 1;
        self.cursor_col = 0;
        self.modified = true;
    }
    
    /// Delete entire line
    pub fn delete_line(&mut self) {
        if self.lines.len() > 1 {
            self.lines.remove(self.cursor_line);
            if self.cursor_line >= self.lines.len() {
                self.cursor_line = self.lines.len() - 1;
            }
        } else {
            self.lines[0].clear();
        }
        self.cursor_col = 0;
        self.modified = true;
    }
    
    /// Get current line content
    pub fn current_line(&self) -> &str {
        &self.lines[self.cursor_line]
    }
    
    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.lines[self.cursor_line].len();
        }
    }
    
    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.cursor_col < self.lines[self.cursor_line].len() {
            self.cursor_col += 1;
        } else if self.cursor_line + 1 < self.lines.len() {
            self.cursor_line += 1;
            self.cursor_col = 0;
        }
    }
    
    /// Move cursor up
    pub fn move_cursor_up(&mut self) {
        if self.cursor_line > 0 {
            self.cursor_line -= 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].len());
        }
    }
    
    /// Move cursor down
    pub fn move_cursor_down(&mut self) {
        if self.cursor_line + 1 < self.lines.len() {
            self.cursor_line += 1;
            self.cursor_col = self.cursor_col.min(self.lines[self.cursor_line].len());
        }
    }
    
    /// Move cursor to start of line
    pub fn move_cursor_line_start(&mut self) {
        self.cursor_col = 0;
    }
    
    /// Move cursor to end of line
    pub fn move_cursor_line_end(&mut self) {
        self.cursor_col = self.lines[self.cursor_line].len();
    }
    
    /// Move cursor to start of buffer
    pub fn move_cursor_buffer_start(&mut self) {
        self.cursor_line = 0;
        self.cursor_col = 0;
    }
    
    /// Move cursor to end of buffer
    pub fn move_cursor_buffer_end(&mut self) {
        self.cursor_line = self.lines.len() - 1;
        self.cursor_col = self.lines[self.cursor_line].len();
    }
    
    /// Start selection at current cursor position
    pub fn start_selection(&mut self) {
        self.selection_start = Some((self.cursor_line, self.cursor_col));
        self.selection_end = Some((self.cursor_line, self.cursor_col));
    }
    
    /// Update selection end to current cursor position
    pub fn update_selection(&mut self) {
        if self.selection_start.is_some() {
            self.selection_end = Some((self.cursor_line, self.cursor_col));
        }
    }
    
    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
    
    /// Get selected text
    pub fn get_selection(&self) -> Option<String> {
        let start = self.selection_start?;
        let end = self.selection_end?;
        
        let (start_line, start_col) = start;
        let (end_line, end_col) = end;
        
        if start_line == end_line {
            let line = &self.lines[start_line];
            let start_col = start_col.min(end_col);
            let end_col = start_col.max(end_col);
            Some(line[start_col..end_col].to_string())
        } else {
            let mut result = String::new();
            for line_idx in start_line..=end_line {
                if line_idx == start_line {
                    result.push_str(&self.lines[line_idx][start_col..]);
                } else if line_idx == end_line {
                    result.push_str(&self.lines[line_idx][..end_col]);
                } else {
                    result.push_str(&self.lines[line_idx]);
                }
                if line_idx < end_line {
                    result.push('\n');
                }
            }
            Some(result)
        }
    }
    
    /// Delete selected text
    pub fn delete_selection(&mut self) -> Option<String> {
        let selected = self.get_selection()?;
        let start = self.selection_start?;
        let end = self.selection_end?;
        
        let (start_line, start_col) = start;
        let (end_line, end_col) = end;
        
        if start_line == end_line {
            let line = &mut self.lines[start_line];
            let start_col = start_col.min(end_col);
            let end_col = start_col.max(end_col);
            line.drain(start_col..end_col);
            self.cursor_col = start_col;
        } else {
            // Remove complete lines in between
            for _ in (start_line + 1)..end_line {
                self.lines.remove(start_line + 1);
            }
            
            // Handle first and last lines
            let end_part = self.lines[start_line + 1][end_col..].to_string();
            self.lines[start_line].truncate(start_col);
            self.lines[start_line].push_str(&end_part);
            self.lines.remove(start_line + 1);
            
            self.cursor_line = start_line;
            self.cursor_col = start_col;
        }
        
        self.clear_selection();
        self.modified = true;
        Some(selected)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}