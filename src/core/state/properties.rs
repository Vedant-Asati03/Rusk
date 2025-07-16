use std::collections::HashMap;
use serde_json::Value;

/// Property system for dynamic configuration
/// This allows plugins to register and manage properties dynamically
pub struct PropertySystem {
    properties: HashMap<String, Value>,
    property_types: HashMap<String, String>,
}

impl PropertySystem {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
            property_types: HashMap::new(),
        }
    }
    
    /// Set a property value
    pub fn set_property(&mut self, key: &str, value: Value) {
        let type_name = match &value {
            Value::Bool(_) => "bool",
            Value::Number(_) => "number", 
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Null => "null",
        };
        
        self.property_types.insert(key.to_string(), type_name.to_string());
        self.properties.insert(key.to_string(), value);
    }
    
    /// Get a property value
    pub fn get_property(&self, key: &str) -> Option<&Value> {
        self.properties.get(key)
    }
    
    /// Get a property as a specific type
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.properties.get(key)?.as_bool()
    }
    
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.properties.get(key)?.as_str()
    }
    
    pub fn get_number(&self, key: &str) -> Option<f64> {
        self.properties.get(key)?.as_f64()
    }
    
    /// Remove a property
    pub fn remove_property(&mut self, key: &str) -> Option<Value> {
        self.property_types.remove(key);
        self.properties.remove(key)
    }
    
    /// Check if property exists
    pub fn has_property(&self, key: &str) -> bool {
        self.properties.contains_key(key)
    }
    
    /// Get all property keys
    pub fn property_keys(&self) -> Vec<&String> {
        self.properties.keys().collect()
    }
    
    /// Get property type
    pub fn property_type(&self, key: &str) -> Option<&String> {
        self.property_types.get(key)
    }
    
    /// Clear all properties
    pub fn clear(&mut self) {
        self.properties.clear();
        self.property_types.clear();
    }
    
    /// Get property count
    pub fn property_count(&self) -> usize {
        self.properties.len()
    }
}