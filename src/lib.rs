//! Rusk - A modular text editor with plugin architecture
//!
//! This crate provides a flexible text editor framework with support for
//! different editing modes (vim, emacs), UI backends (TUI, GUI), and
//! extensible plugin system.

pub mod core;
pub mod plugins;
pub mod services;
pub mod utils;
pub mod error;
pub mod config;

// Re-export commonly used types
pub use error::{Result, RuskError};
pub use config::{Config, ConfigLoader};
pub use core::editor::Editor;
pub use core::state::EditorState;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the editor with default configuration
pub fn init() -> Result<Editor> {
    Editor::new()
}