//! Buffer management domain
//!
//! This module handles text buffers, operations on them,
//! and the undo/redo history system.

pub mod buffer;
pub mod operations;
pub mod history;

pub use buffer::Buffer;