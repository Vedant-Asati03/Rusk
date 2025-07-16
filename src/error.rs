use thiserror::Error;

/// Result type for Rusk operations
pub type Result<T> = std::result::Result<T, RuskError>;

/// Main error type for the Rusk editor
#[derive(Error, Debug)]
pub enum RuskError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Plugin error: {0}")]
    Plugin(String),
    
    #[error("Buffer error: {0}")]
    Buffer(String),
    
    #[error("Cursor error: {0}")]
    Cursor(String),
    
    #[error("UI error: {0}")]
    Ui(String),
    
    #[error("Syntax highlighting error: {0}")]
    Syntax(String),
    
    #[error("Language support error: {0}")]
    Language(String),
    
    #[error("Command error: {0}")]
    Command(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl RuskError {
    /// Create a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }
    
    /// Create a new plugin error
    pub fn plugin(msg: impl Into<String>) -> Self {
        Self::Plugin(msg.into())
    }
    
    /// Create a new buffer error
    pub fn buffer(msg: impl Into<String>) -> Self {
        Self::Buffer(msg.into())
    }
    
    /// Create a new cursor error
    pub fn cursor(msg: impl Into<String>) -> Self {
        Self::Cursor(msg.into())
    }
    
    /// Create a new UI error
    pub fn ui(msg: impl Into<String>) -> Self {
        Self::Ui(msg.into())
    }
    
    /// Create a new syntax error
    pub fn syntax(msg: impl Into<String>) -> Self {
        Self::Syntax(msg.into())
    }
    
    /// Create a new language support error
    pub fn language(msg: impl Into<String>) -> Self {
        Self::Language(msg.into())
    }
    
    /// Create a new command error
    pub fn command(msg: impl Into<String>) -> Self {
        Self::Command(msg.into())
    }
    
    /// Create a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
    
    /// Create a new IO error from string
    pub fn io_string(msg: impl Into<String>) -> Self {
        Self::Io(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
    }
}