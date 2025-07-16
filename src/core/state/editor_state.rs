use crate::{Result, RuskError};
use crate::core::buffer::Buffer;
use super::properties::PropertySystem;

/// Editor state management
pub struct EditorState {
    buffers: Vec<Buffer>,
    current_buffer_index: usize,
    modified: bool,
    properties: PropertySystem,
}

impl EditorState {
    /// Create a new editor state
    pub fn new() -> Result<Self> {
        Ok(Self {
            buffers: Vec::new(),
            current_buffer_index: 0,
            modified: false,
            properties: PropertySystem::new(),
        })
    }
    
    /// Add a new buffer
    pub fn add_buffer(&mut self, buffer: Buffer) {
        self.buffers.push(buffer);
        self.current_buffer_index = self.buffers.len() - 1;
        self.modified = true;
    }
    
    /// Get current buffer
    pub fn current_buffer(&self) -> Option<&Buffer> {
        self.buffers.get(self.current_buffer_index)
    }
    
    /// Get current buffer mutably
    pub fn current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.buffers.get_mut(self.current_buffer_index)
    }
    
    /// Get all buffers
    pub fn buffers(&self) -> &[Buffer] {
        &self.buffers
    }
    
    /// Get all buffers mutably
    pub fn buffers_mut(&mut self) -> &mut [Buffer] {
        &mut self.buffers
    }
    
    /// Get buffer count
    pub fn buffer_count(&self) -> usize {
        self.buffers.len()
    }
    
    /// Switch to next buffer
    pub fn next_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_index = (self.current_buffer_index + 1) % self.buffers.len();
        }
    }
    
    /// Switch to previous buffer
    pub fn previous_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_index = if self.current_buffer_index == 0 {
                self.buffers.len() - 1
            } else {
                self.current_buffer_index - 1
            };
        }
    }
    
    /// Switch to specific buffer by index
    pub fn switch_to_buffer(&mut self, index: usize) -> Result<()> {
        if index < self.buffers.len() {
            self.current_buffer_index = index;
            Ok(())
        } else {
            Err(RuskError::Buffer(format!("Buffer index {} out of range", index)))
        }
    }
    
    /// Close current buffer
    pub fn close_current_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.buffers.remove(self.current_buffer_index);
            
            // Adjust current buffer index
            if self.current_buffer_index >= self.buffers.len() && !self.buffers.is_empty() {
                self.current_buffer_index = self.buffers.len() - 1;
            }
            
            self.modified = true;
        }
    }
    
    /// Close buffer by index
    pub fn close_buffer(&mut self, index: usize) -> Result<()> {
        if index < self.buffers.len() {
            self.buffers.remove(index);
            
            // Adjust current buffer index if necessary
            if index <= self.current_buffer_index && self.current_buffer_index > 0 {
                self.current_buffer_index -= 1;
            }
            
            // If we removed the last buffer and current index is out of bounds
            if self.current_buffer_index >= self.buffers.len() && !self.buffers.is_empty() {
                self.current_buffer_index = self.buffers.len() - 1;
            }
            
            self.modified = true;
            Ok(())
        } else {
            Err(RuskError::Buffer(format!("Buffer index {} out of range", index)))
        }
    }
    
    /// Get current buffer index
    pub fn current_buffer_index(&self) -> usize {
        self.current_buffer_index
    }
    
    /// Check if state is modified
    pub fn is_modified(&self) -> bool {
        self.modified || self.buffers.iter().any(|buffer| buffer.is_modified())
    }
    
    /// Mark state as saved
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }
    
    /// Find buffer by file path
    pub fn find_buffer_by_path(&self, path: &str) -> Option<usize> {
        self.buffers
            .iter()
            .position(|buffer| buffer.file_path() == Some(path))
    }
    
    /// Get buffer names for display
    pub fn buffer_names(&self) -> Vec<String> {
        self.buffers
            .iter()
            .enumerate()
            .map(|(i, buffer)| {
                let name = buffer.file_path()
                    .map(|path| {
                        std::path::Path::new(path)
                            .file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or(path)
                            .to_string()
                    })
                    .unwrap_or_else(|| format!("[Buffer {}]", i + 1));
                
                if buffer.is_modified() {
                    format!("{}*", name)
                } else {
                    name
                }
            })
            .collect()
    }
    
    /// Check if any buffers have unsaved changes
    pub fn has_unsaved_changes(&self) -> bool {
        self.buffers.iter().any(|buffer| buffer.is_modified())
    }
    
    /// Get list of modified buffer indices
    pub fn modified_buffer_indices(&self) -> Vec<usize> {
        self.buffers
            .iter()
            .enumerate()
            .filter(|(_, buffer)| buffer.is_modified())
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Get access to property system
    pub fn properties(&self) -> &PropertySystem {
        &self.properties
    }
    
    /// Get mutable access to property system
    pub fn properties_mut(&mut self) -> &mut PropertySystem {
        &mut self.properties
    }
}