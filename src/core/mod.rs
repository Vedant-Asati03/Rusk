//! Core domain logic for the Rusk editor
//!
//! This module contains the fundamental components that make up the editor:
//! - Editor orchestrator
//! - Buffer management
//! - Cursor management  
//! - State management

pub mod editor;
pub mod buffer;
pub mod cursor;
pub mod state;

// Re-export commonly used types
pub use editor::Editor;
pub use state::EditorState;