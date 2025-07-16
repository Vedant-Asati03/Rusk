//! Vim mode implementation
//!
//! This module implements vim-style editing modes and commands

use crate::plugins::Plugin;
use crate::core::buffer::Buffer;
use crate::{Result, Config};
use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
    VisualLine,
    Command,
}

/// Vim editing plugin with full modal editing support
pub struct VimPlugin {
    name: String,
    version: String,
    mode: VimMode,
    command_buffer: String,
    last_command: String,
    register: HashMap<char, String>,
    config: Option<Config>,
}

impl VimPlugin {
    pub fn new() -> Self {
        Self {
            name: "vim".to_string(),
            version: "0.1.0".to_string(),
            mode: VimMode::Normal,
            command_buffer: String::new(),
            last_command: String::new(),
            register: HashMap::new(),
            config: None,
        }
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: Config) {
        self.config = Some(config);
    }
    
    /// Get current mode
    pub fn mode(&self) -> &VimMode {
        &self.mode
    }
    
    /// Set mode
    pub fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
        if mode != VimMode::Command {
            self.command_buffer.clear();
        }
    }
    
    /// Handle key input based on current mode
    pub fn handle_key(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        match self.mode {
            VimMode::Normal => self.handle_normal_mode(key, buffer),
            VimMode::Insert => self.handle_insert_mode(key, buffer),
            VimMode::Visual => self.handle_visual_mode(key, buffer),
            VimMode::VisualLine => self.handle_visual_line_mode(key, buffer),
            VimMode::Command => self.handle_command_mode(key, buffer),
        }
    }
    
    /// Handle normal mode keys
    fn handle_normal_mode(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        match key.code {
            // Movement
            KeyCode::Char('h') => {
                buffer.move_cursor_left();
                Ok(false)
            }
            KeyCode::Char('j') => {
                buffer.move_cursor_down();
                Ok(false)
            }
            KeyCode::Char('k') => {
                buffer.move_cursor_up();
                Ok(false)
            }
            KeyCode::Char('l') => {
                buffer.move_cursor_right();
                Ok(false)
            }
            KeyCode::Char('w') => {
                self.move_word_forward(buffer);
                Ok(false)
            }
            KeyCode::Char('b') => {
                self.move_word_backward(buffer);
                Ok(false)
            }
            KeyCode::Char('0') => {
                buffer.move_cursor_line_start();
                Ok(false)
            }
            KeyCode::Char('$') => {
                buffer.move_cursor_line_end();
                Ok(false)
            }
            
            // Mode changes
            KeyCode::Char('i') => {
                self.set_mode(VimMode::Insert);
                Ok(false)
            }
            KeyCode::Char('a') => {
                buffer.move_cursor_right();
                self.set_mode(VimMode::Insert);
                Ok(false)
            }
            KeyCode::Char('o') => {
                buffer.move_cursor_line_end();
                buffer.insert_newline();
                self.set_mode(VimMode::Insert);
                Ok(false)
            }
            KeyCode::Char('O') => {
                buffer.move_cursor_line_start();
                buffer.insert_newline();
                buffer.move_cursor_up();
                self.set_mode(VimMode::Insert);
                Ok(false)
            }
            KeyCode::Char('v') => {
                buffer.start_selection();
                self.set_mode(VimMode::Visual);
                Ok(false)
            }
            KeyCode::Char('V') => {
                buffer.start_selection();
                self.set_mode(VimMode::VisualLine);
                Ok(false)
            }
            
            // Editing
            KeyCode::Char('x') => {
                buffer.delete_char();
                Ok(false)
            }
            KeyCode::Char('r') => {
                // TODO: Implement replace mode
                Ok(false)
            }
            KeyCode::Char('d') => {
                // TODO: Implement delete operations (dd, dw, etc.)
                self.handle_delete_command(buffer);
                Ok(false)
            }
            KeyCode::Char('y') => {
                // TODO: Implement yank operations
                self.handle_yank_command(buffer);
                Ok(false)
            }
            KeyCode::Char('p') => {
                self.paste_after(buffer);
                Ok(false)
            }
            KeyCode::Char('P') => {
                self.paste_before(buffer);
                Ok(false)
            }
            KeyCode::Char('u') => {
                // TODO: Implement undo
                Ok(false)
            }
            
            // Command mode
            KeyCode::Char(':') => {
                self.set_mode(VimMode::Command);
                self.command_buffer.push(':');
                Ok(false)
            }
            
            // File operations
            KeyCode::Char('g') => {
                // Handle gg for file start
                if self.last_command == "g" {
                    buffer.move_cursor_buffer_start();
                    self.last_command.clear();
                } else {
                    self.last_command = "g".to_string();
                }
                Ok(false)
            }
            KeyCode::Char('G') => {
                buffer.move_cursor_buffer_end();
                Ok(false)
            }
            
            // Quit
            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                Ok(true) // Signal quit
            }
            
            _ => {
                self.last_command.clear();
                Ok(false)
            }
        }
    }
    
    /// Handle insert mode keys
    fn handle_insert_mode(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            KeyCode::Char(c) if key.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' => {
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            KeyCode::Char(c) => {
                buffer.insert_char(c);
                Ok(false)
            }
            KeyCode::Enter => {
                buffer.insert_newline();
                Ok(false)
            }
            KeyCode::Backspace => {
                buffer.backspace();
                Ok(false)
            }
            KeyCode::Delete => {
                buffer.delete_char();
                Ok(false)
            }
            KeyCode::Tab => {
                // Insert tab or spaces based on config
                let tab_str = if let Some(config) = &self.config {
                    if config.editor.insert_spaces {
                        " ".repeat(config.editor.tab_size)
                    } else {
                        "\t".to_string()
                    }
                } else {
                    "    ".to_string() // Default 4 spaces
                };
                buffer.insert_str(&tab_str);
                Ok(false)
            }
            _ => Ok(false)
        }
    }
    
    /// Handle visual mode keys
    fn handle_visual_mode(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                buffer.clear_selection();
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            
            // Movement updates selection
            KeyCode::Char('h') => {
                buffer.move_cursor_left();
                buffer.update_selection();
                Ok(false)
            }
            KeyCode::Char('j') => {
                buffer.move_cursor_down();
                buffer.update_selection();
                Ok(false)
            }
            KeyCode::Char('k') => {
                buffer.move_cursor_up();
                buffer.update_selection();
                Ok(false)
            }
            KeyCode::Char('l') => {
                buffer.move_cursor_right();
                buffer.update_selection();
                Ok(false)
            }
            
            // Operations on selection
            KeyCode::Char('d') => {
                if let Some(text) = buffer.delete_selection() {
                    self.register.insert('"', text);
                }
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            KeyCode::Char('y') => {
                if let Some(text) = buffer.get_selection() {
                    self.register.insert('"', text);
                }
                buffer.clear_selection();
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            KeyCode::Char('c') => {
                if let Some(text) = buffer.delete_selection() {
                    self.register.insert('"', text);
                }
                self.set_mode(VimMode::Insert);
                Ok(false)
            }
            
            _ => Ok(false)
        }
    }
    
    /// Handle visual line mode keys
    fn handle_visual_line_mode(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        // Similar to visual mode but operates on whole lines
        self.handle_visual_mode(key, buffer)
    }
    
    /// Handle command mode keys
    fn handle_command_mode(&mut self, key: KeyEvent, buffer: &mut Buffer) -> Result<bool> {
        match key.code {
            KeyCode::Esc => {
                self.set_mode(VimMode::Normal);
                Ok(false)
            }
            KeyCode::Enter => {
                let result = self.execute_command(&self.command_buffer.clone(), buffer)?;
                self.set_mode(VimMode::Normal);
                Ok(result)
            }
            KeyCode::Backspace => {
                self.command_buffer.pop();
                if self.command_buffer.is_empty() {
                    self.set_mode(VimMode::Normal);
                }
                Ok(false)
            }
            KeyCode::Char(c) => {
                self.command_buffer.push(c);
                Ok(false)
            }
            _ => Ok(false)
        }
    }
    
    /// Execute command mode commands
    fn execute_command(&mut self, command: &str, buffer: &mut Buffer) -> Result<bool> {
        match command.trim() {
            ":w" => {
                buffer.save()?;
                Ok(false)
            }
            ":q" => {
                if buffer.is_modified() {
                    // TODO: Show warning about unsaved changes
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            ":wq" | ":x" => {
                buffer.save()?;
                Ok(true)
            }
            ":q!" => {
                Ok(true)
            }
            _ if command.starts_with(":w ") => {
                let filename = &command[3..];
                buffer.save_as(filename)?;
                Ok(false)
            }
            _ => {
                // Unknown command
                Ok(false)
            }
        }
    }
    
    /// Move cursor forward by word
    fn move_word_forward(&self, buffer: &mut Buffer) {
        let (line, col) = buffer.cursor_position();
        if let Some(current_line) = buffer.get_line(line) {
            let chars: Vec<char> = current_line.chars().collect();
            let mut new_col = col;
            
            // Skip current word
            while new_col < chars.len() && chars[new_col].is_alphanumeric() {
                new_col += 1;
            }
            
            // Skip whitespace
            while new_col < chars.len() && chars[new_col].is_whitespace() {
                new_col += 1;
            }
            
            if new_col >= chars.len() && line + 1 < buffer.line_count() {
                buffer.set_cursor_position(line + 1, 0);
            } else {
                buffer.set_cursor_position(line, new_col);
            }
        }
    }
    
    /// Move cursor backward by word
    fn move_word_backward(&self, buffer: &mut Buffer) {
        let (line, col) = buffer.cursor_position();
        if col > 0 {
            if let Some(current_line) = buffer.get_line(line) {
                let chars: Vec<char> = current_line.chars().collect();
                let mut new_col = col.saturating_sub(1);
                
                // Skip whitespace
                while new_col > 0 && chars[new_col].is_whitespace() {
                    new_col -= 1;
                }
                
                // Skip current word
                while new_col > 0 && chars[new_col].is_alphanumeric() {
                    new_col -= 1;
                }
                
                if new_col > 0 || !chars[new_col].is_alphanumeric() {
                    new_col += 1;
                }
                
                buffer.set_cursor_position(line, new_col);
            }
        } else if line > 0 {
            let prev_line = line - 1;
            if let Some(prev_line_content) = buffer.get_line(prev_line) {
                buffer.set_cursor_position(prev_line, prev_line_content.len());
            }
        }
    }
    
    /// Handle delete command (dd, dw, etc.)
    fn handle_delete_command(&mut self, buffer: &mut Buffer) {
        // For now, just implement dd (delete line)
        if self.last_command == "d" {
            let line_content = buffer.current_line().to_string();
            self.register.insert('"', line_content);
            buffer.delete_line();
            self.last_command.clear();
        } else {
            self.last_command = "d".to_string();
        }
    }
    
    /// Handle yank command (yy, yw, etc.)
    fn handle_yank_command(&mut self, buffer: &mut Buffer) {
        // For now, just implement yy (yank line)
        if self.last_command == "y" {
            let line_content = buffer.current_line().to_string();
            self.register.insert('"', line_content);
            self.last_command.clear();
        } else {
            self.last_command = "y".to_string();
        }
    }
    
    /// Paste after cursor
    fn paste_after(&mut self, buffer: &mut Buffer) {
        if let Some(text) = self.register.get(&'"').cloned() {
            if text.contains('\n') {
                // Paste as new line
                buffer.move_cursor_line_end();
                buffer.insert_newline();
                buffer.insert_str(&text);
            } else {
                // Paste inline
                buffer.insert_str(&text);
            }
        }
    }
    
    /// Paste before cursor
    fn paste_before(&mut self, buffer: &mut Buffer) {
        if let Some(text) = self.register.get(&'"').cloned() {
            if text.contains('\n') {
                // Paste as new line above
                buffer.move_cursor_line_start();
                buffer.insert_str(&text);
                buffer.insert_newline();
                buffer.move_cursor_up();
            } else {
                // Paste inline before cursor
                buffer.insert_str(&text);
            }
        }
    }
    
    /// Get command buffer for display
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }
}

impl Plugin for VimPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn initialize(&mut self) -> Result<()> {
        println!("Initializing Vim plugin");
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        println!("Shutting down Vim plugin");
        Ok(())
    }
}