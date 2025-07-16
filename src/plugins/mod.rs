//! Plugin system for the Rusk editor
//!
//! This module provides the plugin architecture that allows extending
//! the editor with different editing modes, UI backends, language support,
//! and other functionality.

pub mod registry;
pub mod events;
pub mod implementations;

// Re-export plugin traits
pub use registry::{Plugin, PluginRegistry};