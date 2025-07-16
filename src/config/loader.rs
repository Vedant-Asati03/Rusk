use crate::{Result, RuskError};
use super::{Config, ThemeManager, ColorScheme};
use std::fs;
use std::path::PathBuf;

pub struct ConfigLoader;

impl ConfigLoader {
    /// Load configuration from ~/.config/rusk.config
    pub fn load() -> Result<Config> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)
                .map_err(|e| RuskError::io_string(format!("Failed to read config file: {}", e)))?;
            
            let config: Config = toml::from_str(&content)
                .map_err(|e| RuskError::Config(format!("Failed to parse config file: {}", e)))?;
            
            println!("Loaded configuration from: {}", config_path.display());
            Ok(config)
        } else {
            println!("Config file not found, creating default configuration");
            let default_config = Config::default();
            Self::save(&default_config)?;
            Ok(default_config)
        }
    }
    
    /// Save configuration to ~/.config/rusk.config
    pub fn save(config: &Config) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| RuskError::io_string(format!("Failed to create config directory: {}", e)))?;
        }
        
        let content = toml::to_string_pretty(config)
            .map_err(|e| RuskError::Config(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, content)
            .map_err(|e| RuskError::io_string(format!("Failed to write config file: {}", e)))?;
        
        println!("Configuration saved to: {}", config_path.display());
        Ok(())
    }
    
    /// Get the path to the configuration file
    fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| RuskError::Config("Could not determine home directory".to_string()))?;
        
        Ok(home_dir.join(".config").join("rusk").join("rusk.config"))
    }
    
    /// Load configuration from specific file path
    pub fn load_from_file(file_path: &str) -> Result<Config> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| RuskError::io_string(format!("Failed to read config file '{}': {}", file_path, e)))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| RuskError::Config(format!("Failed to parse config file '{}': {}", file_path, e)))?;
        
        Ok(config)
    }

    /// Reload configuration from file
    pub fn reload() -> Result<Config> {
        Self::load()
    }
    
    /// Check if configuration file exists
    pub fn config_exists() -> Result<bool> {
        let config_path = Self::get_config_path()?;
        Ok(config_path.exists())
    }
    
    /// Get configuration file path as string
    pub fn get_config_path_string() -> Result<String> {
        let path = Self::get_config_path()?;
        Ok(path.to_string_lossy().to_string())
    }
    
    /// Load the color scheme for a given config
    pub fn load_color_scheme(config: &Config) -> Result<ColorScheme> {
        ThemeManager::load_theme(&config.ui.theme)
    }
    
    /// Get available themes
    pub fn list_available_themes() -> Result<Vec<String>> {
        ThemeManager::list_themes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::env;
    
    #[test]
    fn test_config_path() {
        let result = ConfigLoader::get_config_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".config"));
        assert!(path.to_string_lossy().contains("rusk.config"));
    }
    
    #[test]
    fn test_default_config_creation() {
        let config = Config::default();
        assert_eq!(config.editor.tab_size, 4);
        assert!(config.editor.vim_mode);
        assert!(config.ui.show_status_bar);
        assert!(!config.plugins.enabled_plugins.is_empty());
    }
}