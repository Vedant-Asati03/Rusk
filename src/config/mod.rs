pub mod loader;
pub mod settings;
pub mod themes;

pub use loader::ConfigLoader;
pub use settings::{Config, EditorSettings, KeyBindings, UiSettings, PluginSettings, ColorScheme, CursorStyle};
pub use themes::ThemeManager;