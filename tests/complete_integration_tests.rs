use rusk::{Editor, ConfigLoader, Result};

#[test]
fn test_editor_creation() -> Result<()> {
    let editor = Editor::new()?;
    assert_eq!(editor.buffer_count(), 0);
    Ok(())
}

#[test]
fn test_editor_with_file() -> Result<()> {
    // Create a test file
    std::fs::write("test_file.txt", "Hello, World!")?;
    
    let editor = Editor::with_file("test_file.txt")?;
    assert_eq!(editor.buffer_count(), 1);
    
    // Cleanup
    std::fs::remove_file("test_file.txt").ok();
    Ok(())
}

#[test]
fn test_configuration_loading() -> Result<()> {
    let config = ConfigLoader::load()?;
    assert_eq!(config.editor.tab_size, 4);
    assert!(config.editor.vim_mode);
    Ok(())
}

#[test]
fn test_buffer_operations() -> Result<()> {
    use rusk::core::buffer::Buffer;
    
    let mut buffer = Buffer::new();
    
    // Test basic operations
    buffer.insert_char('H');
    buffer.insert_char('i');
    assert_eq!(buffer.current_line(), "Hi");
    
    buffer.insert_newline();
    buffer.insert_str("World");
    assert_eq!(buffer.line_count(), 2);
    
    // Test cursor movement
    buffer.move_cursor_up();
    assert_eq!(buffer.cursor_position().0, 0);
    
    Ok(())
}

#[test]
fn test_vim_plugin() -> Result<()> {
    use rusk::plugins::implementations::editing::vim::{VimPlugin, VimMode};
    use rusk::plugins::Plugin;
    
    let mut vim = VimPlugin::new();
    vim.initialize()?;
    
    assert_eq!(vim.mode(), &VimMode::Normal);
    
    vim.shutdown()?;
    Ok(())
}