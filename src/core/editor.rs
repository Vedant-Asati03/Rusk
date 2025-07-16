use crate::{Result, RuskError, Config, ConfigLoader};
use crate::core::buffer::Buffer;
use crate::plugins::implementations::editing::vim::VimPlugin;
use crate::plugins::implementations::ui::tui::TuiPlugin;
use crate::plugins::Plugin;
use super::state::EditorState;

/// Main editor orchestrator that coordinates all components
pub struct Editor {
    state: EditorState,
    config: Config,
    vim_plugin: VimPlugin,
    tui_plugin: TuiPlugin,
}

impl Editor {
    /// Create a new editor instance
    pub fn new() -> Result<Self> {
        // Load configuration
        let config = ConfigLoader::load()?;
        
        // Create state
        let state = EditorState::new()?;
        
        // Create plugins
        let mut vim_plugin = VimPlugin::new();
        vim_plugin.set_config(config.clone());
        
        let mut tui_plugin = TuiPlugin::new();
        tui_plugin.set_config(config.clone());
        
        Ok(Self {
            state,
            config,
            vim_plugin,
            tui_plugin,
        })
    }
    
    /// Create editor with specific file
    pub fn with_file(file_path: &str) -> Result<Self> {
        let mut editor = Self::new()?;
        editor.open_file(file_path)?;
        Ok(editor)
    }
    
    /// Get a reference to the editor state
    pub fn state(&self) -> &EditorState {
        &self.state
    }
    
    /// Get a mutable reference to the editor state
    pub fn state_mut(&mut self) -> &mut EditorState {
        &mut self.state
    }
    
    /// Get configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Open a file in the editor
    pub fn open_file(&mut self, file_path: &str) -> Result<()> {
        let buffer = Buffer::from_file(file_path)?;
        self.state.add_buffer(buffer);
        Ok(())
    }
    
    /// Create a new empty buffer
    pub fn new_buffer(&mut self) -> Result<()> {
        let buffer = Buffer::new();
        self.state.add_buffer(buffer);
        Ok(())
    }
    
    /// Save current buffer
    pub fn save_current_buffer(&mut self) -> Result<()> {
        if let Some(buffer) = self.state.current_buffer_mut() {
            buffer.save()?;
        }
        Ok(())
    }
    
    /// Save current buffer as
    pub fn save_current_buffer_as(&mut self, file_path: &str) -> Result<()> {
        if let Some(buffer) = self.state.current_buffer_mut() {
            buffer.save_as(file_path)?;
        }
        Ok(())
    }
    
    /// Close current buffer
    pub fn close_current_buffer(&mut self) -> Result<bool> {
        if let Some(buffer) = self.state.current_buffer() {
            if buffer.is_modified() {
                // TODO: Prompt user to save changes
                return Ok(false);
            }
        }
        
        self.state.close_current_buffer();
        Ok(self.state.buffer_count() == 0) // Return true if no more buffers
    }
    
    /// Run the main editor loop
    pub fn run(&mut self) -> Result<()> {
        // Initialize plugins
        self.vim_plugin.initialize()?;
        self.tui_plugin.initialize()?;
        
        // Ensure we have at least one buffer
        if self.state.buffer_count() == 0 {
            self.new_buffer()?;
        }
        
        // Run the main event loop
        let result = self.run_main_loop();
        
        // Shutdown plugins
        let _ = self.tui_plugin.shutdown();
        let _ = self.vim_plugin.shutdown();
        
        result
    }
    
    /// Run the main event loop
    fn run_main_loop(&mut self) -> Result<()> {
        loop {
            // Get current buffer
            let current_buffer = self.state.current_buffer_mut()
                .ok_or_else(|| RuskError::Internal("No active buffer".to_string()))?;
            
            // Run TUI event loop which handles vim input
            match self.tui_plugin.run_event_loop(current_buffer, &mut self.vim_plugin) {
                Ok(()) => break, // Normal exit
                Err(e) => {
                    // Handle errors gracefully
                    eprintln!("Error in event loop: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Get current buffer
    pub fn current_buffer(&self) -> Option<&Buffer> {
        self.state.current_buffer()
    }
    
    /// Get current buffer mutably
    pub fn current_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.state.current_buffer_mut()
    }
    
    /// Switch to next buffer
    pub fn next_buffer(&mut self) {
        self.state.next_buffer();
    }
    
    /// Switch to previous buffer
    pub fn previous_buffer(&mut self) {
        self.state.previous_buffer();
    }
    
    /// Get buffer count
    pub fn buffer_count(&self) -> usize {
        self.state.buffer_count()
    }
    
    /// Reload configuration
    pub fn reload_config(&mut self) -> Result<()> {
        self.config = ConfigLoader::reload()?;
        self.vim_plugin.set_config(self.config.clone());
        self.tui_plugin.set_config(self.config.clone());
        Ok(())
    }
    
    /// Check if any buffers are modified
    pub fn has_unsaved_changes(&self) -> bool {
        self.state.buffers().iter().any(|buffer| buffer.is_modified())
    }
    
    /// Get list of modified files
    pub fn modified_files(&self) -> Vec<String> {
        self.state.buffers()
            .iter()
            .filter(|buffer| buffer.is_modified())
            .filter_map(|buffer| buffer.file_path().map(|s| s.to_string()))
            .collect()
    }
    
    /// Force quit without saving
    pub fn force_quit(&mut self) -> Result<()> {
        // Just shutdown plugins and exit
        let _ = self.tui_plugin.shutdown();
        let _ = self.vim_plugin.shutdown();
        Ok(())
    }
    
    /// Quit with save check
    pub fn quit(&mut self) -> Result<bool> {
        if self.has_unsaved_changes() {
            // Return false to indicate quit was cancelled due to unsaved changes
            Ok(false)
        } else {
            self.force_quit()?;
            Ok(true)
        }
    }
}