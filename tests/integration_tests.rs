use rusk::{init, Result};
use rusk::plugins::{PluginRegistry};
use rusk::plugins::implementations::editing::vim::VimPlugin;
use rusk::plugins::implementations::ui::tui::TuiPlugin;

#[test]
fn test_editor_initialization() -> Result<()> {
    let _editor = init()?;
    Ok(())
}

#[test]
fn test_plugin_system() -> Result<()> {
    let mut registry = PluginRegistry::new();
    
    // Register plugins
    registry.register(Box::new(VimPlugin::new()))?;
    registry.register(Box::new(TuiPlugin::new()))?;
    
    assert_eq!(registry.plugin_count(), 2);
    
    // Initialize all plugins
    registry.initialize_all()?;
    
    Ok(())
}

#[test]
fn test_buffer_creation() {
    use rusk::core::buffer::buffer::Buffer;
    
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert!(!buffer.is_modified());
}

#[test]
fn test_cursor_movement() {
    use rusk::core::cursor::{position::Position, movement::Movement};
    
    let pos = Position::new(5, 10);
    
    let up = Movement::up(pos);
    assert_eq!(up.line, 4);
    assert_eq!(up.column, 10);
    
    let down = Movement::down(pos);
    assert_eq!(down.line, 6);
    assert_eq!(down.column, 10);
    
    let left = Movement::left(pos);
    assert_eq!(left.line, 5);
    assert_eq!(left.column, 9);
    
    let right = Movement::right(pos);
    assert_eq!(right.line, 5);
    assert_eq!(right.column, 11);
}