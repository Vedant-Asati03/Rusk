use crate::Result;

/// Trait that all plugins must implement
pub trait Plugin {
    /// Plugin name
    fn name(&self) -> &str;
    
    /// Plugin version
    fn version(&self) -> &str;
    
    /// Initialize the plugin
    fn initialize(&mut self) -> Result<()>;
    
    /// Shutdown the plugin
    fn shutdown(&mut self) -> Result<()>;
}

/// Plugin registry that manages all loaded plugins
pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    /// Create a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    
    /// Register a new plugin
    pub fn register(&mut self, plugin: Box<dyn Plugin>) -> Result<()> {
        println!("Registering plugin: {}", plugin.name());
        self.plugins.push(plugin);
        Ok(())
    }
    
    /// Initialize all registered plugins
    pub fn initialize_all(&mut self) -> Result<()> {
        for plugin in &mut self.plugins {
            plugin.initialize()?;
        }
        Ok(())
    }
    
    /// Get the number of registered plugins
    pub fn plugin_count(&self) -> usize {
        self.plugins.len()
    }
}