use std::fs;
use std::path::PathBuf;
use crate::{Result, RuskError};

/// Theme manager for handling built-in and custom themes
pub struct ThemeManager;

impl ThemeManager {
    /// Load a theme by name (checks built-in first, then custom themes)
    pub fn load_theme(theme_name: &str) -> Result<ColorScheme> {
        // First check built-in themes
        if let Some(theme) = Self::get_builtin_theme(theme_name) {
            return Ok(theme);
        }
        
        // Then check custom themes
        Self::load_custom_theme(theme_name)
    }
    
    /// Get a built-in theme by name
    pub fn get_builtin_theme(theme_name: &str) -> Option<ColorScheme> {
        match theme_name {
            "dark" | "default" => Some(Self::dark_theme()),
            "light" => Some(Self::light_theme()),
            "dracula" => Some(Self::dracula_theme()),
            "monokai" => Some(Self::monokai_theme()),
            "solarized_dark" => Some(Self::solarized_dark_theme()),
            "solarized_light" => Some(Self::solarized_light_theme()),
            "gruvbox_dark" => Some(Self::gruvbox_dark_theme()),
            "gruvbox_light" => Some(Self::gruvbox_light_theme()),
            "nord" => Some(Self::nord_theme()),
            "one_dark" => Some(Self::one_dark_theme()),
            _ => None,
        }
    }
    
    /// Load a custom theme from ~/.config/rusk/themes/
    pub fn load_custom_theme(theme_name: &str) -> Result<ColorScheme> {
        let theme_path = Self::get_custom_theme_path(theme_name)?;
        
        if !theme_path.exists() {
            return Err(RuskError::Config(format!("Theme '{}' not found", theme_name)));
        }
        
        let content = fs::read_to_string(&theme_path)
            .map_err(|e| RuskError::io_string(format!("Failed to read theme file: {}", e)))?;
        
        let theme: ColorScheme = serde_json::from_str(&content)
            .map_err(|e| RuskError::Config(format!("Failed to parse theme file: {}", e)))?;
        
        Ok(theme)
    }
    
    /// Get the path to a custom theme file
    fn get_custom_theme_path(theme_name: &str) -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| RuskError::Config("Could not determine home directory".to_string()))?;
        
        let theme_filename = if theme_name.ends_with(".json") {
            theme_name.to_string()
        } else {
            format!("{}.json", theme_name)
        };
        
        Ok(home_dir.join(".config").join("rusk").join("themes").join(theme_filename))
    }
    
    /// List all available themes (built-in and custom)
    pub fn list_themes() -> Result<Vec<String>> {
        let mut themes = vec![
            "dark".to_string(),
            "light".to_string(),
            "dracula".to_string(),
            "monokai".to_string(),
            "solarized_dark".to_string(),
            "solarized_light".to_string(),
            "gruvbox_dark".to_string(),
            "gruvbox_light".to_string(),
            "nord".to_string(),
            "one_dark".to_string(),
        ];
        
        // Add custom themes
        if let Ok(custom_themes) = Self::list_custom_themes() {
            themes.extend(custom_themes);
        }
        
        Ok(themes)
    }
    
    /// List custom themes from ~/.config/rusk/themes/
    pub fn list_custom_themes() -> Result<Vec<String>> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| RuskError::Config("Could not determine home directory".to_string()))?;
        
        let themes_dir = home_dir.join(".config").join("rusk").join("themes");
        
        if !themes_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut themes = Vec::new();
        
        for entry in fs::read_dir(themes_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    themes.push(name.to_string());
                }
            }
        }
        
        Ok(themes)
    }
    
    /// Save a custom theme
    pub fn save_custom_theme(theme_name: &str, theme: &ColorScheme) -> Result<()> {
        let theme_path = Self::get_custom_theme_path(theme_name)?;
        
        // Create themes directory if it doesn't exist
        if let Some(parent) = theme_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| RuskError::io_string(format!("Failed to create themes directory: {}", e)))?;
        }
        
        let content = serde_json::to_string_pretty(theme)
            .map_err(|e| RuskError::Config(format!("Failed to serialize theme: {}", e)))?;
        
        fs::write(&theme_path, content)
            .map_err(|e| RuskError::io_string(format!("Failed to write theme file: {}", e)))?;
        
        Ok(())
    }

    // Built-in theme definitions
    
    /// Dark theme (VS Code Dark+ inspired)
    pub fn dark_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#1e1e1e".to_string(),
            foreground: "#d4d4d4".to_string(),
            cursor: "#ffffff".to_string(),
            selection: "#264f78".to_string(),
            line_number: "#858585".to_string(),
            current_line: "#2a2d2e".to_string(),
            status_bar: "#007acc".to_string(),
            border: "#3e3e3e".to_string(),
            
            // Comments
            comment: "#6a9955".to_string(),
            comment_doc: "#608b4e".to_string(),
            
            // Keywords
            keyword: "#569cd6".to_string(),
            keyword_control: "#c586c0".to_string(),
            keyword_function: "#569cd6".to_string(),
            keyword_storage: "#569cd6".to_string(),
            keyword_import: "#c586c0".to_string(),
            
            // Types and Identifiers
            type_name: "#4ec9b0".to_string(),
            type_builtin: "#569cd6".to_string(),
            type_parameter: "#4ec9b0".to_string(),
            variable: "#9cdcfe".to_string(),
            variable_builtin: "#569cd6".to_string(),
            variable_parameter: "#9cdcfe".to_string(),
            constant: "#4fc1ff".to_string(),
            constant_builtin: "#569cd6".to_string(),
            
            // Functions and Methods
            function: "#dcdcaa".to_string(),
            function_builtin: "#dcdcaa".to_string(),
            function_method: "#dcdcaa".to_string(),
            function_macro: "#4ec9b0".to_string(),
            
            // Literals
            string: "#ce9178".to_string(),
            string_escape: "#d7ba7d".to_string(),
            string_interpolation: "#569cd6".to_string(),
            number: "#b5cea8".to_string(),
            number_float: "#b5cea8".to_string(),
            boolean: "#569cd6".to_string(),
            
            // Operators and Punctuation
            operator: "#d4d4d4".to_string(),
            operator_logical: "#569cd6".to_string(),
            punctuation: "#d4d4d4".to_string(),
            punctuation_bracket: "#ffd700".to_string(),
            punctuation_delimiter: "#d4d4d4".to_string(),
            
            // Language-specific
            attribute: "#9cdcfe".to_string(),
            property: "#9cdcfe".to_string(),
            tag: "#569cd6".to_string(),
            tag_attribute: "#92c5f7".to_string(),
            label: "#4fc1ff".to_string(),
            
            // Special
            error: "#f44747".to_string(),
            warning: "#ffcc02".to_string(),
            todo: "#ffcc02".to_string(),
            emphasis: "#d4d4d4".to_string(),
            strong: "#d4d4d4".to_string(),
            link: "#4fc1ff".to_string(),
        }
    }
    
    /// Light theme
    pub fn light_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            cursor: "#000000".to_string(),
            selection: "#add6ff".to_string(),
            line_number: "#237893".to_string(),
            current_line: "#f5f5f5".to_string(),
            status_bar: "#0078d4".to_string(),
            border: "#e5e5e5".to_string(),
            
            // Comments
            comment: "#008000".to_string(),
            comment_doc: "#008000".to_string(),
            
            // Keywords
            keyword: "#0000ff".to_string(),
            keyword_control: "#af00db".to_string(),
            keyword_function: "#0000ff".to_string(),
            keyword_storage: "#0000ff".to_string(),
            keyword_import: "#af00db".to_string(),
            
            // Types and Identifiers
            type_name: "#267f99".to_string(),
            type_builtin: "#0000ff".to_string(),
            type_parameter: "#267f99".to_string(),
            variable: "#001080".to_string(),
            variable_builtin: "#0000ff".to_string(),
            variable_parameter: "#001080".to_string(),
            constant: "#0070c1".to_string(),
            constant_builtin: "#0000ff".to_string(),
            
            // Functions and Methods
            function: "#795e26".to_string(),
            function_builtin: "#795e26".to_string(),
            function_method: "#795e26".to_string(),
            function_macro: "#267f99".to_string(),
            
            // Literals
            string: "#a31515".to_string(),
            string_escape: "#ee0000".to_string(),
            string_interpolation: "#0000ff".to_string(),
            number: "#098658".to_string(),
            number_float: "#098658".to_string(),
            boolean: "#0000ff".to_string(),
            
            // Operators and Punctuation
            operator: "#000000".to_string(),
            operator_logical: "#0000ff".to_string(),
            punctuation: "#000000".to_string(),
            punctuation_bracket: "#0431fa".to_string(),
            punctuation_delimiter: "#000000".to_string(),
            
            // Language-specific
            attribute: "#001080".to_string(),
            property: "#001080".to_string(),
            tag: "#800000".to_string(),
            tag_attribute: "#e50000".to_string(),
            label: "#0070c1".to_string(),
            
            // Special
            error: "#cd3131".to_string(),
            warning: "#bf8803".to_string(),
            todo: "#bf8803".to_string(),
            emphasis: "#000000".to_string(),
            strong: "#000000".to_string(),
            link: "#0070c1".to_string(),
        }
    }
    
    /// Dracula theme
    pub fn dracula_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#282a36".to_string(),
            foreground: "#f8f8f2".to_string(),
            cursor: "#f8f8f0".to_string(),
            selection: "#44475a".to_string(),
            line_number: "#6272a4".to_string(),
            current_line: "#44475a".to_string(),
            status_bar: "#bd93f9".to_string(),
            border: "#6272a4".to_string(),
            
            // Comments
            comment: "#6272a4".to_string(),
            comment_doc: "#6272a4".to_string(),
            
            // Keywords
            keyword: "#ff79c6".to_string(),
            keyword_control: "#ff79c6".to_string(),
            keyword_function: "#ff79c6".to_string(),
            keyword_storage: "#ff79c6".to_string(),
            keyword_import: "#ff79c6".to_string(),
            
            // Types and Identifiers
            type_name: "#8be9fd".to_string(),
            type_builtin: "#8be9fd".to_string(),
            type_parameter: "#8be9fd".to_string(),
            variable: "#f8f8f2".to_string(),
            variable_builtin: "#bd93f9".to_string(),
            variable_parameter: "#ffb86c".to_string(),
            constant: "#bd93f9".to_string(),
            constant_builtin: "#bd93f9".to_string(),
            
            // Functions and Methods
            function: "#50fa7b".to_string(),
            function_builtin: "#50fa7b".to_string(),
            function_method: "#50fa7b".to_string(),
            function_macro: "#50fa7b".to_string(),
            
            // Literals
            string: "#f1fa8c".to_string(),
            string_escape: "#ff79c6".to_string(),
            string_interpolation: "#ff79c6".to_string(),
            number: "#bd93f9".to_string(),
            number_float: "#bd93f9".to_string(),
            boolean: "#bd93f9".to_string(),
            
            // Operators and Punctuation
            operator: "#ff79c6".to_string(),
            operator_logical: "#ff79c6".to_string(),
            punctuation: "#f8f8f2".to_string(),
            punctuation_bracket: "#f8f8f2".to_string(),
            punctuation_delimiter: "#f8f8f2".to_string(),
            
            // Language-specific
            attribute: "#50fa7b".to_string(),
            property: "#f8f8f2".to_string(),
            tag: "#ff79c6".to_string(),
            tag_attribute: "#50fa7b".to_string(),
            label: "#8be9fd".to_string(),
            
            // Special
            error: "#ff5555".to_string(),
            warning: "#ffb86c".to_string(),
            todo: "#ffb86c".to_string(),
            emphasis: "#f8f8f2".to_string(),
            strong: "#f8f8f2".to_string(),
            link: "#8be9fd".to_string(),
        }
    }
    
    /// Monokai theme
    pub fn monokai_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#272822".to_string(),
            foreground: "#f8f8f2".to_string(),
            cursor: "#f8f8f0".to_string(),
            selection: "#49483e".to_string(),
            line_number: "#90908a".to_string(),
            current_line: "#3e3d32".to_string(),
            status_bar: "#66d9ef".to_string(),
            border: "#49483e".to_string(),
            
            // Comments
            comment: "#75715e".to_string(),
            comment_doc: "#75715e".to_string(),
            
            // Keywords
            keyword: "#f92672".to_string(),
            keyword_control: "#f92672".to_string(),
            keyword_function: "#f92672".to_string(),
            keyword_storage: "#f92672".to_string(),
            keyword_import: "#f92672".to_string(),
            
            // Types and Identifiers
            type_name: "#66d9ef".to_string(),
            type_builtin: "#66d9ef".to_string(),
            type_parameter: "#66d9ef".to_string(),
            variable: "#f8f8f2".to_string(),
            variable_builtin: "#66d9ef".to_string(),
            variable_parameter: "#fd971f".to_string(),
            constant: "#ae81ff".to_string(),
            constant_builtin: "#ae81ff".to_string(),
            
            // Functions and Methods
            function: "#a6e22e".to_string(),
            function_builtin: "#a6e22e".to_string(),
            function_method: "#a6e22e".to_string(),
            function_macro: "#a6e22e".to_string(),
            
            // Literals
            string: "#e6db74".to_string(),
            string_escape: "#ae81ff".to_string(),
            string_interpolation: "#ae81ff".to_string(),
            number: "#ae81ff".to_string(),
            number_float: "#ae81ff".to_string(),
            boolean: "#ae81ff".to_string(),
            
            // Operators and Punctuation
            operator: "#f92672".to_string(),
            operator_logical: "#f92672".to_string(),
            punctuation: "#f8f8f2".to_string(),
            punctuation_bracket: "#f8f8f2".to_string(),
            punctuation_delimiter: "#f8f8f2".to_string(),
            
            // Language-specific
            attribute: "#a6e22e".to_string(),
            property: "#f8f8f2".to_string(),
            tag: "#f92672".to_string(),
            tag_attribute: "#a6e22e".to_string(),
            label: "#66d9ef".to_string(),
            
            // Special
            error: "#f92672".to_string(),
            warning: "#fd971f".to_string(),
            todo: "#fd971f".to_string(),
            emphasis: "#f8f8f2".to_string(),
            strong: "#f8f8f2".to_string(),
            link: "#66d9ef".to_string(),
        }
    }
    
    /// Solarized Dark theme
    pub fn solarized_dark_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#002b36".to_string(),
            foreground: "#839496".to_string(),
            cursor: "#93a1a1".to_string(),
            selection: "#073642".to_string(),
            line_number: "#586e75".to_string(),
            current_line: "#073642".to_string(),
            status_bar: "#268bd2".to_string(),
            border: "#073642".to_string(),
            
            // Comments
            comment: "#586e75".to_string(),
            comment_doc: "#586e75".to_string(),
            
            // Keywords
            keyword: "#859900".to_string(),
            keyword_control: "#859900".to_string(),
            keyword_function: "#859900".to_string(),
            keyword_storage: "#859900".to_string(),
            keyword_import: "#859900".to_string(),
            
            // Types and Identifiers
            type_name: "#268bd2".to_string(),
            type_builtin: "#268bd2".to_string(),
            type_parameter: "#268bd2".to_string(),
            variable: "#839496".to_string(),
            variable_builtin: "#268bd2".to_string(),
            variable_parameter: "#b58900".to_string(),
            constant: "#2aa198".to_string(),
            constant_builtin: "#2aa198".to_string(),
            
            // Functions and Methods
            function: "#268bd2".to_string(),
            function_builtin: "#268bd2".to_string(),
            function_method: "#268bd2".to_string(),
            function_macro: "#268bd2".to_string(),
            
            // Literals
            string: "#2aa198".to_string(),
            string_escape: "#dc322f".to_string(),
            string_interpolation: "#dc322f".to_string(),
            number: "#2aa198".to_string(),
            number_float: "#2aa198".to_string(),
            boolean: "#2aa198".to_string(),
            
            // Operators and Punctuation
            operator: "#859900".to_string(),
            operator_logical: "#859900".to_string(),
            punctuation: "#839496".to_string(),
            punctuation_bracket: "#839496".to_string(),
            punctuation_delimiter: "#839496".to_string(),
            
            // Language-specific
            attribute: "#268bd2".to_string(),
            property: "#839496".to_string(),
            tag: "#859900".to_string(),
            tag_attribute: "#268bd2".to_string(),
            label: "#268bd2".to_string(),
            
            // Special
            error: "#dc322f".to_string(),
            warning: "#b58900".to_string(),
            todo: "#b58900".to_string(),
            emphasis: "#839496".to_string(),
            strong: "#839496".to_string(),
            link: "#268bd2".to_string(),
        }
    }
    
    /// Solarized Light theme
    pub fn solarized_light_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#fdf6e3".to_string(),
            foreground: "#657b83".to_string(),
            cursor: "#586e75".to_string(),
            selection: "#eee8d5".to_string(),
            line_number: "#93a1a1".to_string(),
            current_line: "#eee8d5".to_string(),
            status_bar: "#268bd2".to_string(),
            border: "#eee8d5".to_string(),
            
            // Comments
            comment: "#93a1a1".to_string(),
            comment_doc: "#93a1a1".to_string(),
            
            // Keywords
            keyword: "#859900".to_string(),
            keyword_control: "#859900".to_string(),
            keyword_function: "#859900".to_string(),
            keyword_storage: "#859900".to_string(),
            keyword_import: "#859900".to_string(),
            
            // Types and Identifiers
            type_name: "#268bd2".to_string(),
            type_builtin: "#268bd2".to_string(),
            type_parameter: "#268bd2".to_string(),
            variable: "#657b83".to_string(),
            variable_builtin: "#268bd2".to_string(),
            variable_parameter: "#b58900".to_string(),
            constant: "#2aa198".to_string(),
            constant_builtin: "#2aa198".to_string(),
            
            // Functions and Methods
            function: "#268bd2".to_string(),
            function_builtin: "#268bd2".to_string(),
            function_method: "#268bd2".to_string(),
            function_macro: "#268bd2".to_string(),
            
            // Literals
            string: "#2aa198".to_string(),
            string_escape: "#dc322f".to_string(),
            string_interpolation: "#dc322f".to_string(),
            number: "#2aa198".to_string(),
            number_float: "#2aa198".to_string(),
            boolean: "#2aa198".to_string(),
            
            // Operators and Punctuation
            operator: "#859900".to_string(),
            operator_logical: "#859900".to_string(),
            punctuation: "#657b83".to_string(),
            punctuation_bracket: "#657b83".to_string(),
            punctuation_delimiter: "#657b83".to_string(),
            
            // Language-specific
            attribute: "#268bd2".to_string(),
            property: "#657b83".to_string(),
            tag: "#859900".to_string(),
            tag_attribute: "#268bd2".to_string(),
            label: "#268bd2".to_string(),
            
            // Special
            error: "#dc322f".to_string(),
            warning: "#b58900".to_string(),
            todo: "#b58900".to_string(),
            emphasis: "#657b83".to_string(),
            strong: "#657b83".to_string(),
            link: "#268bd2".to_string(),
        }
    }
    
    /// Gruvbox Dark theme
    pub fn gruvbox_dark_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#282828".to_string(),
            foreground: "#ebdbb2".to_string(),
            cursor: "#ebdbb2".to_string(),
            selection: "#504945".to_string(),
            line_number: "#7c6f64".to_string(),
            current_line: "#3c3836".to_string(),
            status_bar: "#458588".to_string(),
            border: "#504945".to_string(),
            
            // Comments
            comment: "#928374".to_string(),
            comment_doc: "#928374".to_string(),
            
            // Keywords
            keyword: "#fb4934".to_string(),
            keyword_control: "#fb4934".to_string(),
            keyword_function: "#fb4934".to_string(),
            keyword_storage: "#fb4934".to_string(),
            keyword_import: "#fb4934".to_string(),
            
            // Types and Identifiers
            type_name: "#fabd2f".to_string(),
            type_builtin: "#fabd2f".to_string(),
            type_parameter: "#fabd2f".to_string(),
            variable: "#ebdbb2".to_string(),
            variable_builtin: "#d3869b".to_string(),
            variable_parameter: "#fe8019".to_string(),
            constant: "#d3869b".to_string(),
            constant_builtin: "#d3869b".to_string(),
            
            // Functions and Methods
            function: "#b8bb26".to_string(),
            function_builtin: "#b8bb26".to_string(),
            function_method: "#b8bb26".to_string(),
            function_macro: "#8ec07c".to_string(),
            
            // Literals
            string: "#b8bb26".to_string(),
            string_escape: "#fe8019".to_string(),
            string_interpolation: "#fe8019".to_string(),
            number: "#d3869b".to_string(),
            number_float: "#d3869b".to_string(),
            boolean: "#d3869b".to_string(),
            
            // Operators and Punctuation
            operator: "#fe8019".to_string(),
            operator_logical: "#fe8019".to_string(),
            punctuation: "#ebdbb2".to_string(),
            punctuation_bracket: "#ebdbb2".to_string(),
            punctuation_delimiter: "#ebdbb2".to_string(),
            
            // Language-specific
            attribute: "#8ec07c".to_string(),
            property: "#ebdbb2".to_string(),
            tag: "#fb4934".to_string(),
            tag_attribute: "#b8bb26".to_string(),
            label: "#458588".to_string(),
            
            // Special
            error: "#fb4934".to_string(),
            warning: "#fabd2f".to_string(),
            todo: "#fabd2f".to_string(),
            emphasis: "#ebdbb2".to_string(),
            strong: "#ebdbb2".to_string(),
            link: "#458588".to_string(),
        }
    }
    
    /// Gruvbox Light theme
    pub fn gruvbox_light_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#fbf1c7".to_string(),
            foreground: "#3c3836".to_string(),
            cursor: "#3c3836".to_string(),
            selection: "#ebdbb2".to_string(),
            line_number: "#7c6f64".to_string(),
            current_line: "#f2e5bc".to_string(),
            status_bar: "#076678".to_string(),
            border: "#ebdbb2".to_string(),
            
            // Comments
            comment: "#928374".to_string(),
            comment_doc: "#928374".to_string(),
            
            // Keywords
            keyword: "#9d0006".to_string(),
            keyword_control: "#9d0006".to_string(),
            keyword_function: "#9d0006".to_string(),
            keyword_storage: "#9d0006".to_string(),
            keyword_import: "#9d0006".to_string(),
            
            // Types and Identifiers
            type_name: "#b57614".to_string(),
            type_builtin: "#b57614".to_string(),
            type_parameter: "#b57614".to_string(),
            variable: "#3c3836".to_string(),
            variable_builtin: "#8f3f71".to_string(),
            variable_parameter: "#af3a03".to_string(),
            constant: "#8f3f71".to_string(),
            constant_builtin: "#8f3f71".to_string(),
            
            // Functions and Methods
            function: "#79740e".to_string(),
            function_builtin: "#79740e".to_string(),
            function_method: "#79740e".to_string(),
            function_macro: "#427b58".to_string(),
            
            // Literals
            string: "#79740e".to_string(),
            string_escape: "#af3a03".to_string(),
            string_interpolation: "#af3a03".to_string(),
            number: "#8f3f71".to_string(),
            number_float: "#8f3f71".to_string(),
            boolean: "#8f3f71".to_string(),
            
            // Operators and Punctuation
            operator: "#af3a03".to_string(),
            operator_logical: "#af3a03".to_string(),
            punctuation: "#3c3836".to_string(),
            punctuation_bracket: "#3c3836".to_string(),
            punctuation_delimiter: "#3c3836".to_string(),
            
            // Language-specific
            attribute: "#427b58".to_string(),
            property: "#3c3836".to_string(),
            tag: "#9d0006".to_string(),
            tag_attribute: "#79740e".to_string(),
            label: "#076678".to_string(),
            
            // Special
            error: "#9d0006".to_string(),
            warning: "#b57614".to_string(),
            todo: "#b57614".to_string(),
            emphasis: "#3c3836".to_string(),
            strong: "#3c3836".to_string(),
            link: "#076678".to_string(),
        }
    }
    
    /// Nord theme
    pub fn nord_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#2e3440".to_string(),
            foreground: "#d8dee9".to_string(),
            cursor: "#d8dee9".to_string(),
            selection: "#434c5e".to_string(),
            line_number: "#4c566a".to_string(),
            current_line: "#3b4252".to_string(),
            status_bar: "#5e81ac".to_string(),
            border: "#434c5e".to_string(),
            
            // Comments
            comment: "#616e88".to_string(),
            comment_doc: "#616e88".to_string(),
            
            // Keywords
            keyword: "#81a1c1".to_string(),
            keyword_control: "#81a1c1".to_string(),
            keyword_function: "#81a1c1".to_string(),
            keyword_storage: "#81a1c1".to_string(),
            keyword_import: "#81a1c1".to_string(),
            
            // Types and Identifiers
            type_name: "#8fbcbb".to_string(),
            type_builtin: "#8fbcbb".to_string(),
            type_parameter: "#8fbcbb".to_string(),
            variable: "#d8dee9".to_string(),
            variable_builtin: "#81a1c1".to_string(),
            variable_parameter: "#d08770".to_string(),
            constant: "#b48ead".to_string(),
            constant_builtin: "#b48ead".to_string(),
            
            // Functions and Methods
            function: "#88c0d0".to_string(),
            function_builtin: "#88c0d0".to_string(),
            function_method: "#88c0d0".to_string(),
            function_macro: "#8fbcbb".to_string(),
            
            // Literals
            string: "#a3be8c".to_string(),
            string_escape: "#ebcb8b".to_string(),
            string_interpolation: "#ebcb8b".to_string(),
            number: "#b48ead".to_string(),
            number_float: "#b48ead".to_string(),
            boolean: "#b48ead".to_string(),
            
            // Operators and Punctuation
            operator: "#81a1c1".to_string(),
            operator_logical: "#81a1c1".to_string(),
            punctuation: "#eceff4".to_string(),
            punctuation_bracket: "#eceff4".to_string(),
            punctuation_delimiter: "#eceff4".to_string(),
            
            // Language-specific
            attribute: "#8fbcbb".to_string(),
            property: "#d8dee9".to_string(),
            tag: "#81a1c1".to_string(),
            tag_attribute: "#8fbcbb".to_string(),
            label: "#5e81ac".to_string(),
            
            // Special
            error: "#bf616a".to_string(),
            warning: "#ebcb8b".to_string(),
            todo: "#ebcb8b".to_string(),
            emphasis: "#d8dee9".to_string(),
            strong: "#d8dee9".to_string(),
            link: "#5e81ac".to_string(),
        }
    }
    
    /// One Dark theme (Atom inspired)
    pub fn one_dark_theme() -> ColorScheme {
        ColorScheme {
            // UI Colors
            background: "#282c34".to_string(),
            foreground: "#abb2bf".to_string(),
            cursor: "#528bff".to_string(),
            selection: "#3e4451".to_string(),
            line_number: "#636d83".to_string(),
            current_line: "#2c313c".to_string(),
            status_bar: "#61afef".to_string(),
            border: "#181a1f".to_string(),
            
            // Comments
            comment: "#5c6370".to_string(),
            comment_doc: "#5c6370".to_string(),
            
            // Keywords
            keyword: "#c678dd".to_string(),
            keyword_control: "#c678dd".to_string(),
            keyword_function: "#c678dd".to_string(),
            keyword_storage: "#c678dd".to_string(),
            keyword_import: "#c678dd".to_string(),
            
            // Types and Identifiers
            type_name: "#e5c07b".to_string(),
            type_builtin: "#e06c75".to_string(),
            type_parameter: "#e5c07b".to_string(),
            variable: "#e06c75".to_string(),
            variable_builtin: "#c678dd".to_string(),
            variable_parameter: "#d19a66".to_string(),
            constant: "#d19a66".to_string(),
            constant_builtin: "#56b6c2".to_string(),
            
            // Functions and Methods
            function: "#61afef".to_string(),
            function_builtin: "#61afef".to_string(),
            function_method: "#61afef".to_string(),
            function_macro: "#e06c75".to_string(),
            
            // Literals
            string: "#98c379".to_string(),
            string_escape: "#56b6c2".to_string(),
            string_interpolation: "#56b6c2".to_string(),
            number: "#d19a66".to_string(),
            number_float: "#d19a66".to_string(),
            boolean: "#56b6c2".to_string(),
            
            // Operators and Punctuation
            operator: "#56b6c2".to_string(),
            operator_logical: "#56b6c2".to_string(),
            punctuation: "#abb2bf".to_string(),
            punctuation_bracket: "#abb2bf".to_string(),
            punctuation_delimiter: "#abb2bf".to_string(),
            
            // Language-specific
            attribute: "#e5c07b".to_string(),
            property: "#e06c75".to_string(),
            tag: "#e06c75".to_string(),
            tag_attribute: "#d19a66".to_string(),
            label: "#61afef".to_string(),
            
            // Special
            error: "#e06c75".to_string(),
            warning: "#e5c07b".to_string(),
            todo: "#e5c07b".to_string(),
            emphasis: "#abb2bf".to_string(),
            strong: "#abb2bf".to_string(),
            link: "#61afef".to_string(),
        }
    }
}

/// Re-export ColorScheme from settings
pub use super::settings::ColorScheme;