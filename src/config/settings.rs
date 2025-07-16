use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ratatui::style::Color;

/// Utility functions for color conversion
impl ColorScheme {
    /// Parse a hex color string to ratatui Color
    pub fn parse_color(hex: &str) -> Color {
        if hex.starts_with('#') && hex.len() == 7 {
            if let Ok(value) = u32::from_str_radix(&hex[1..], 16) {
                let r = ((value >> 16) & 0xFF) as u8;
                let g = ((value >> 8) & 0xFF) as u8;
                let b = (value & 0xFF) as u8;
                return Color::Rgb(r, g, b);
            }
        }
        
        // Fallback to named colors or default
        match hex.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "gray" | "grey" => Color::Gray,
            "darkgray" | "darkgrey" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            _ => Color::White, // Default fallback
        }
    }
    
    /// Get background color as ratatui Color
    pub fn background_color(&self) -> Color {
        Self::parse_color(&self.background)
    }
    
    /// Get foreground color as ratatui Color
    pub fn foreground_color(&self) -> Color {
        Self::parse_color(&self.foreground)
    }
    
    /// Get cursor color as ratatui Color
    pub fn cursor_color(&self) -> Color {
        Self::parse_color(&self.cursor)
    }
    
    /// Get selection color as ratatui Color
    pub fn selection_color(&self) -> Color {
        Self::parse_color(&self.selection)
    }
    
    /// Get line number color as ratatui Color
    pub fn line_number_color(&self) -> Color {
        Self::parse_color(&self.line_number)
    }
    
    /// Get current line color as ratatui Color
    pub fn current_line_color(&self) -> Color {
        Self::parse_color(&self.current_line)
    }
    
    /// Get status bar color as ratatui Color
    pub fn status_bar_color(&self) -> Color {
        Self::parse_color(&self.status_bar)
    }
    
    /// Get border color as ratatui Color
    pub fn border_color(&self) -> Color {
        Self::parse_color(&self.border)
    }
    
    /// Get comment color as ratatui Color
    pub fn comment_color(&self) -> Color {
        Self::parse_color(&self.comment)
    }
    
    /// Get keyword color as ratatui Color
    pub fn keyword_color(&self) -> Color {
        Self::parse_color(&self.keyword)
    }
    
    /// Get string color as ratatui Color
    pub fn string_color(&self) -> Color {
        Self::parse_color(&self.string)
    }
    
    /// Get number color as ratatui Color
    pub fn number_color(&self) -> Color {
        Self::parse_color(&self.number)
    }
    
    /// Get operator color as ratatui Color
    pub fn operator_color(&self) -> Color {
        Self::parse_color(&self.operator)
    }
    
    /// Get function color as ratatui Color
    pub fn function_color(&self) -> Color {
        Self::parse_color(&self.function)
    }
    
    /// Get type name color as ratatui Color
    pub fn type_name_color(&self) -> Color {
        Self::parse_color(&self.type_name)
    }
    
    /// Get variable color as ratatui Color
    pub fn variable_color(&self) -> Color {
        Self::parse_color(&self.variable)
    }
    
    /// Get constant color as ratatui Color
    pub fn constant_color(&self) -> Color {
        Self::parse_color(&self.constant)
    }
    
    /// Get error color as ratatui Color
    pub fn error_color(&self) -> Color {
        Self::parse_color(&self.error)
    }
    
    /// Get warning color as ratatui Color
    pub fn warning_color(&self) -> Color {
        Self::parse_color(&self.warning)
    }
    
    /// Get punctuation bracket color as ratatui Color
    pub fn punctuation_bracket_color(&self) -> Color {
        Self::parse_color(&self.punctuation_bracket)
    }
    
    /// Get keyword control color as ratatui Color
    pub fn keyword_control_color(&self) -> Color {
        Self::parse_color(&self.keyword_control)
    }
    
    /// Get keyword function color as ratatui Color
    pub fn keyword_function_color(&self) -> Color {
        Self::parse_color(&self.keyword_function)
    }
    
    /// Get keyword import color as ratatui Color
    pub fn keyword_import_color(&self) -> Color {
        Self::parse_color(&self.keyword_import)
    }
    
    /// Get string escape color as ratatui Color
    pub fn string_escape_color(&self) -> Color {
        Self::parse_color(&self.string_escape)
    }
    
    /// Get boolean color as ratatui Color
    pub fn boolean_color(&self) -> Color {
        Self::parse_color(&self.boolean)
    }
    
    /// Get constant builtin color as ratatui Color
    pub fn constant_builtin_color(&self) -> Color {
        Self::parse_color(&self.constant_builtin)
    }
    
    /// Get function builtin color as ratatui Color
    pub fn function_builtin_color(&self) -> Color {
        Self::parse_color(&self.function_builtin)
    }
    
    /// Get function method color as ratatui Color
    pub fn function_method_color(&self) -> Color {
        Self::parse_color(&self.function_method)
    }
    
    /// Get type builtin color as ratatui Color
    pub fn type_builtin_color(&self) -> Color {
        Self::parse_color(&self.type_builtin)
    }
    
    /// Get variable builtin color as ratatui Color
    pub fn variable_builtin_color(&self) -> Color {
        Self::parse_color(&self.variable_builtin)
    }
    
    /// Get variable parameter color as ratatui Color
    pub fn variable_parameter_color(&self) -> Color {
        Self::parse_color(&self.variable_parameter)
    }
    
    /// Get punctuation color as ratatui Color
    pub fn punctuation_color(&self) -> Color {
        Self::parse_color(&self.punctuation)
    }
    
    /// Get punctuation delimiter color as ratatui Color
    pub fn punctuation_delimiter_color(&self) -> Color {
        Self::parse_color(&self.punctuation_delimiter)
    }
    
    /// Get attribute color as ratatui Color
    pub fn attribute_color(&self) -> Color {
        Self::parse_color(&self.attribute)
    }
    
    /// Get property color as ratatui Color
    pub fn property_color(&self) -> Color {
        Self::parse_color(&self.property)
    }
    
    /// Get label color as ratatui Color
    pub fn label_color(&self) -> Color {
        Self::parse_color(&self.label)
    }
    
    /// Get tag color as ratatui Color
    pub fn tag_color(&self) -> Color {
        Self::parse_color(&self.tag)
    }
    
    /// Get text color (foreground) as ratatui Color
    pub fn text_color(&self) -> Color {
        Self::parse_color(&self.foreground)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub editor: EditorSettings,
    pub ui: UiSettings,
    pub keybindings: KeyBindings,
    pub plugins: PluginSettings,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            editor: EditorSettings::default(),
            ui: UiSettings::default(),
            keybindings: KeyBindings::default(),
            plugins: PluginSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    pub tab_size: usize,
    pub insert_spaces: bool,
    pub line_numbers: bool,
    pub wrap_lines: bool,
    pub auto_save: bool,
    pub auto_save_interval: u64, // seconds
    pub backup_files: bool,
    pub show_whitespace: bool,
    pub highlight_current_line: bool,
    pub vim_mode: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            tab_size: 4,
            insert_spaces: true,
            line_numbers: true,
            wrap_lines: false,
            auto_save: false,
            auto_save_interval: 30,
            backup_files: true,
            show_whitespace: false,
            highlight_current_line: true,
            vim_mode: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: String,
    pub font_size: u16,
    pub show_status_bar: bool,
    pub show_tab_bar: bool,
    pub cursor_style: CursorStyle,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14,
            show_status_bar: true,
            show_tab_bar: true,
            cursor_style: CursorStyle::Block,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorStyle {
    Block,
    Line,
    Underline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    // UI Colors
    pub background: String,
    pub foreground: String,
    pub cursor: String,
    pub selection: String,
    pub line_number: String,
    pub current_line: String,
    pub status_bar: String,
    pub border: String,
    
    // Syntax Highlighting Colors
    pub comment: String,
    pub comment_doc: String,
    
    // Keywords
    pub keyword: String,
    pub keyword_control: String,  // if, else, for, while
    pub keyword_function: String, // fn, def, function
    pub keyword_storage: String,  // let, const, var, mut
    pub keyword_import: String,   // import, use, from
    
    // Types and Identifiers
    pub type_name: String,
    pub type_builtin: String,    // str, int, bool, etc.
    pub type_parameter: String,  // Generic type parameters
    pub variable: String,
    pub variable_builtin: String, // self, this, super
    pub variable_parameter: String, // Function parameters
    pub constant: String,
    pub constant_builtin: String, // true, false, null, None
    
    // Functions and Methods
    pub function: String,
    pub function_builtin: String, // print, len, etc.
    pub function_method: String,  // Object methods
    pub function_macro: String,   // Rust macros, etc.
    
    // Literals
    pub string: String,
    pub string_escape: String,    // \n, \t, etc.
    pub string_interpolation: String, // ${} in templates
    pub number: String,
    pub number_float: String,
    pub boolean: String,
    
    // Operators and Punctuation
    pub operator: String,
    pub operator_logical: String, // &&, ||, !
    pub punctuation: String,      // , ; : etc.
    pub punctuation_bracket: String, // () [] {}
    pub punctuation_delimiter: String, // . ->
    
    // Language-specific
    pub attribute: String,        // Rust #[derive], Python @decorator
    pub property: String,         // Object properties
    pub tag: String,             // HTML/XML tags
    pub tag_attribute: String,   // HTML attributes
    pub label: String,           // Goto labels, lifetimes
    
    // Special
    pub error: String,
    pub warning: String,
    pub todo: String,
    pub emphasis: String,        // *italic* in markdown
    pub strong: String,          // **bold** in markdown
    pub link: String,            // URLs and links
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            // UI Colors - Dark theme inspired by VS Code Dark+
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub normal_mode: HashMap<String, String>,
    pub insert_mode: HashMap<String, String>,
    pub visual_mode: HashMap<String, String>,
    pub command_mode: HashMap<String, String>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut normal_mode = HashMap::new();
        
        // Movement
        normal_mode.insert("h".to_string(), "move_left".to_string());
        normal_mode.insert("j".to_string(), "move_down".to_string());
        normal_mode.insert("k".to_string(), "move_up".to_string());
        normal_mode.insert("l".to_string(), "move_right".to_string());
        normal_mode.insert("w".to_string(), "move_word_forward".to_string());
        normal_mode.insert("b".to_string(), "move_word_backward".to_string());
        normal_mode.insert("0".to_string(), "move_line_start".to_string());
        normal_mode.insert("$".to_string(), "move_line_end".to_string());
        normal_mode.insert("gg".to_string(), "move_file_start".to_string());
        normal_mode.insert("G".to_string(), "move_file_end".to_string());
        
        // Editing
        normal_mode.insert("i".to_string(), "enter_insert_mode".to_string());
        normal_mode.insert("a".to_string(), "append".to_string());
        normal_mode.insert("o".to_string(), "open_line_below".to_string());
        normal_mode.insert("O".to_string(), "open_line_above".to_string());
        normal_mode.insert("x".to_string(), "delete_char".to_string());
        normal_mode.insert("dd".to_string(), "delete_line".to_string());
        normal_mode.insert("yy".to_string(), "yank_line".to_string());
        normal_mode.insert("p".to_string(), "paste".to_string());
        normal_mode.insert("u".to_string(), "undo".to_string());
        normal_mode.insert("Ctrl+r".to_string(), "redo".to_string());
        
        // Visual mode
        normal_mode.insert("v".to_string(), "enter_visual_mode".to_string());
        normal_mode.insert("V".to_string(), "enter_visual_line_mode".to_string());
        
        // File operations
        normal_mode.insert(":w".to_string(), "save_file".to_string());
        normal_mode.insert(":q".to_string(), "quit".to_string());
        normal_mode.insert(":wq".to_string(), "save_and_quit".to_string());
        normal_mode.insert(":q!".to_string(), "quit_force".to_string());
        
        let mut insert_mode = HashMap::new();
        insert_mode.insert("Escape".to_string(), "enter_normal_mode".to_string());
        insert_mode.insert("Ctrl+c".to_string(), "enter_normal_mode".to_string());
        
        let mut visual_mode = HashMap::new();
        visual_mode.insert("Escape".to_string(), "enter_normal_mode".to_string());
        visual_mode.insert("d".to_string(), "delete_selection".to_string());
        visual_mode.insert("y".to_string(), "yank_selection".to_string());
        visual_mode.insert("c".to_string(), "change_selection".to_string());
        
        let mut command_mode = HashMap::new();
        command_mode.insert("Enter".to_string(), "execute_command".to_string());
        command_mode.insert("Escape".to_string(), "enter_normal_mode".to_string());
        
        Self {
            normal_mode,
            insert_mode,
            visual_mode,
            command_mode,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginSettings {
    pub enabled_plugins: Vec<String>,
    pub plugin_configs: HashMap<String, toml::Value>,
}

impl Default for PluginSettings {
    fn default() -> Self {
        Self {
            enabled_plugins: vec![
                "vim".to_string(),
                "tui".to_string(),
                "syntax".to_string(),
                "file_io".to_string(),
            ],
            plugin_configs: HashMap::new(),
        }
    }
}