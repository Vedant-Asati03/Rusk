//! TUI backend plugin
//!
//! This module implements terminal user interface functionality using ratatui

use crate::plugins::Plugin;
use crate::core::buffer::Buffer;
use crate::plugins::implementations::editing::vim::{VimPlugin, VimMode};
use crate::plugins::implementations::language::syntax::SyntaxHighlighter;
use crate::{Result, RuskError, Config};
use crate::config::ConfigLoader;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Stdout};

/// TUI backend plugin with full terminal interface
pub struct TuiPlugin {
    name: String,
    version: String,
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
    config: Option<Config>,
    syntax_highlighter: Option<SyntaxHighlighter>,
    scroll_offset: usize,
    status_message: String,
    show_line_numbers: bool,
}

impl TuiPlugin {
    pub fn new() -> Self {
        Self {
            name: "tui".to_string(),
            version: "0.1.0".to_string(),
            terminal: None,
            config: None,
            syntax_highlighter: None,
            scroll_offset: 0,
            status_message: String::new(),
            show_line_numbers: true,
        }
    }
    
    /// Set configuration
    pub fn set_config(&mut self, config: Config) {
        self.show_line_numbers = config.editor.line_numbers;
        
        // Initialize syntax highlighter with theme colors
        if let Ok(mut highlighter) = SyntaxHighlighter::new() {
            if let Ok(color_scheme) = ConfigLoader::load_color_scheme(&config) {
                highlighter.update_color_scheme(&color_scheme);
            }
            self.syntax_highlighter = Some(highlighter);
        }
        
        self.config = Some(config);
    }
    
    /// Setup terminal for TUI mode
    pub fn setup_terminal(&mut self) -> Result<()> {
        enable_raw_mode()
            .map_err(|e| RuskError::Ui(format!("Failed to enable raw mode: {}", e)))?;
        
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .map_err(|e| RuskError::Ui(format!("Failed to setup terminal: {}", e)))?;
        
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)
            .map_err(|e| RuskError::Ui(format!("Failed to create terminal: {}", e)))?;
        
        self.terminal = Some(terminal);
        Ok(())
    }
    
    /// Restore terminal to normal mode
    pub fn restore_terminal(&mut self) -> Result<()> {
        if let Some(mut terminal) = self.terminal.take() {
            disable_raw_mode()
                .map_err(|e| RuskError::Ui(format!("Failed to disable raw mode: {}", e)))?;
            
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )
            .map_err(|e| RuskError::Ui(format!("Failed to restore terminal: {}", e)))?;
            
            terminal.show_cursor()
                .map_err(|e| RuskError::Ui(format!("Failed to show cursor: {}", e)))?;
        }
        Ok(())
    }
    
    /// Main event loop for the TUI
    pub fn run_event_loop(&mut self, buffer: &mut Buffer, vim: &mut VimPlugin) -> Result<()> {
        if self.terminal.is_none() {
            return Err(RuskError::Ui("Terminal not initialized".to_string()));
        }
        
        loop {
            // Draw the interface
            if let Some(terminal) = &mut self.terminal {
                let scroll_offset = self.scroll_offset;
                let show_line_numbers = self.show_line_numbers;
                let status_message = self.status_message.clone();
                
                terminal.draw(|f| {
                    let size = f.area();
                    
                    // Create layout
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(1),      // Editor area
                            Constraint::Length(1),   // Status line
                            Constraint::Length(1),   // Command line
                        ])
                        .split(size);
                    
                    // Get color scheme from config
                    let color_scheme = if let Some(config) = &self.config {
                        ConfigLoader::load_color_scheme(config).unwrap_or_else(|_| {
                            crate::config::settings::ColorScheme::default()
                        })
                    } else {
                        crate::config::settings::ColorScheme::default()
                    };
                    
                    // Draw editor area
                    Self::draw_editor_static(f, chunks[0], buffer, vim, scroll_offset, show_line_numbers, &color_scheme, &mut self.syntax_highlighter);
                    
                    // Draw status line
                    Self::draw_status_line_static(f, chunks[1], buffer, vim, &status_message, &color_scheme);
                    
                    // Draw command line
                    Self::draw_command_line_static(f, chunks[2], vim, &color_scheme);
                })
                    .map_err(|e| RuskError::Ui(format!("Failed to draw: {}", e)))?;
            }
            
            // Handle events
            if event::poll(std::time::Duration::from_millis(100))
                .map_err(|e| RuskError::Ui(format!("Failed to poll events: {}", e)))?
            {
                if let Event::Key(key) = event::read()
                    .map_err(|e| RuskError::Ui(format!("Failed to read event: {}", e)))?
                {
                    // Handle vim key input
                    let should_quit = vim.handle_key(key, buffer)?;
                    if should_quit {
                        break;
                    }
                    
                    // Update scroll if needed
                    self.update_scroll(buffer);
                    
                    // Update status message
                    self.update_status_message(buffer, vim);
                }
            }
        }
        
        Ok(())
    }
    

    
    /// Draw the main editor area (static version)
    fn draw_editor_static(
        f: &mut Frame, 
        area: Rect, 
        buffer: &Buffer, 
        _vim: &VimPlugin, 
        scroll_offset: usize, 
        show_line_numbers: bool, 
        color_scheme: &crate::config::settings::ColorScheme,
        syntax_highlighter: &mut Option<SyntaxHighlighter>
    ) {
        let (cursor_line, cursor_col) = buffer.cursor_position();
        
        // Calculate visible area
        let editor_height = area.height as usize;
        let start_line = scroll_offset;
        let _end_line = (start_line + editor_height).min(buffer.line_count());
        
        // Prepare lines for display
        let mut lines = Vec::new();
        
        for (line_idx, line_content) in buffer.lines().iter().enumerate().skip(start_line).take(editor_height) {
            let mut spans = Vec::new();
            
            // Add line number if enabled
            if show_line_numbers {
                let line_num = format!("{:4} ", line_idx + 1);
                spans.push(Span::styled(
                    line_num,
                    Style::default().fg(color_scheme.line_number_color()),
                ));
            }
            
            // Add line content with syntax highlighting
            let content_spans = if let Some(highlighter) = syntax_highlighter {
                Self::apply_syntax_highlighting(line_content, buffer.language(), highlighter, color_scheme)
            } else {
                // Fallback to plain text
                vec![Span::styled(line_content.clone(), Style::default().fg(color_scheme.foreground_color()))]
            };
            
            // Add line content with cursor highlighting
            if line_idx == cursor_line {
                // Highlight current line
                let bg_style = Style::default().bg(color_scheme.current_line_color());
                
                // Apply cursor highlighting to syntax-highlighted spans
                let cursor_highlighted_spans = Self::apply_cursor_highlighting(
                    content_spans, 
                    cursor_col, 
                    bg_style,
                    color_scheme
                );
                spans.extend(cursor_highlighted_spans);
            } else {
                // Regular line with syntax highlighting
                spans.extend(content_spans);
            }
            
            lines.push(Line::from(spans));
        }
        
        // Create editor widget
        let editor = Paragraph::new(lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Rusk Editor")
                .border_style(Style::default().fg(color_scheme.border_color()))
            )
            .style(Style::default().bg(color_scheme.background_color()))
            .wrap(Wrap { trim: false });
        
        f.render_widget(editor, area);
    }
    
    /// Apply syntax highlighting to a line of text
    fn apply_syntax_highlighting(
        line_content: &str,
        language: &crate::plugins::implementations::language::syntax::SupportedLanguage,
        highlighter: &mut SyntaxHighlighter,
        color_scheme: &crate::config::settings::ColorScheme,
    ) -> Vec<Span<'static>> {
        match highlighter.highlight_line(line_content, language.clone()) {
            Ok(highlights) => {
                if highlights.is_empty() {
                    // No highlights, return plain text
                    return vec![Span::styled(line_content.to_string(), Style::default().fg(color_scheme.foreground_color()))];
                }
                
                let mut spans = Vec::new();
                let mut current_pos = 0;
                
                for highlight in highlights {
                    // Add unhighlighted text before this highlight
                    if highlight.start > current_pos {
                        spans.push(Span::styled(
                            line_content[current_pos..highlight.start].to_string(),
                            Style::default().fg(color_scheme.foreground_color()),
                        ));
                    }
                    
                    // Add highlighted text
                    if highlight.end > highlight.start && highlight.end <= line_content.len() {
                        spans.push(Span::styled(
                            line_content[highlight.start..highlight.end].to_string(),
                            highlight.style,
                        ));
                        current_pos = highlight.end;
                    }
                }
                
                // Add remaining unhighlighted text
                if current_pos < line_content.len() {
                    spans.push(Span::styled(
                        line_content[current_pos..].to_string(),
                        Style::default().fg(color_scheme.foreground_color()),
                    ));
                }
                
                spans
            }
            Err(_e) => {
                // Fallback to plain text on error
                vec![Span::styled(line_content.to_string(), Style::default().fg(color_scheme.foreground_color()))]
            }
        }
    }
    
    /// Apply cursor highlighting to syntax-highlighted spans
    fn apply_cursor_highlighting(
        spans: Vec<Span<'static>>,
        cursor_col: usize,
        bg_style: Style,
        color_scheme: &crate::config::settings::ColorScheme,
    ) -> Vec<Span<'static>> {
        let mut result = Vec::new();
        let mut current_pos = 0;
        let mut cursor_applied = false;
        
        for span in spans {
            let span_text = span.content.as_ref();
            let span_end = current_pos + span_text.len();
            
            if cursor_col >= current_pos && cursor_col < span_end && !cursor_applied {
                // Cursor is within this span
                let cursor_offset = cursor_col - current_pos;
                
                // Before cursor
                if cursor_offset > 0 {
                    result.push(Span::styled(
                        span_text[..cursor_offset].to_string(),
                        span.style.bg(bg_style.bg.unwrap_or(ratatui::style::Color::Reset)),
                    ));
                }
                
                // Cursor character
                let cursor_char = span_text.chars().nth(cursor_offset).unwrap_or(' ');
                result.push(Span::styled(
                    cursor_char.to_string(),
                    Style::default().bg(color_scheme.cursor_color()).fg(color_scheme.background_color()),
                ));
                
                // After cursor in this span
                if cursor_offset + 1 < span_text.len() {
                    result.push(Span::styled(
                        span_text[cursor_offset + 1..].to_string(),
                        span.style.bg(bg_style.bg.unwrap_or(ratatui::style::Color::Reset)),
                    ));
                }
                
                cursor_applied = true;
            } else {
                // Apply background style to entire span
                result.push(Span::styled(
                    span_text.to_string(),
                    span.style.bg(bg_style.bg.unwrap_or(ratatui::style::Color::Reset)),
                ));
            }
            
            current_pos = span_end;
        }
        
        // If cursor is at the end of the line, add a space with cursor style
        if cursor_col >= current_pos && !cursor_applied {
            result.push(Span::styled(
                " ",
                Style::default().bg(color_scheme.cursor_color()).fg(color_scheme.background_color()),
            ));
        }
        
        result
    }
    
    /// Draw the status line (static version)
    fn draw_status_line_static(f: &mut Frame, area: Rect, buffer: &Buffer, vim: &VimPlugin, status_message: &str, color_scheme: &crate::config::settings::ColorScheme) {
        let (line, col) = buffer.cursor_position();
        let mode_str = match vim.mode() {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::VisualLine => "V-LINE",
            VimMode::Command => "COMMAND",
        };
        
        let file_name = buffer.file_path().unwrap_or("[No Name]");
        let modified = if buffer.is_modified() { " [+]" } else { "" };
        let position = format!("{}:{}", line + 1, col + 1);
        let total_lines = buffer.line_count();
        
        let status_text = format!(
            " {} | {} | {}{} | {}/{} lines",
            mode_str, file_name, modified, 
            if !status_message.is_empty() { 
                format!(" | {}", status_message) 
            } else { 
                String::new() 
            },
            position, total_lines
        );
        
        let status_style = match vim.mode() {
            VimMode::Normal => Style::default().bg(color_scheme.status_bar_color()).fg(color_scheme.foreground_color()),
            VimMode::Insert => Style::default().bg(crate::config::settings::ColorScheme::parse_color("#16a085")).fg(color_scheme.background_color()),
            VimMode::Visual | VimMode::VisualLine => Style::default().bg(crate::config::settings::ColorScheme::parse_color("#f39c12")).fg(color_scheme.background_color()),
            VimMode::Command => Style::default().bg(crate::config::settings::ColorScheme::parse_color("#9b59b6")).fg(color_scheme.foreground_color()),
        };
        
        let status = Paragraph::new(status_text)
            .style(status_style);
        
        f.render_widget(status, area);
    }
    
    /// Draw the command line (static version)
    fn draw_command_line_static(f: &mut Frame, area: Rect, vim: &VimPlugin, color_scheme: &crate::config::settings::ColorScheme) {
        let command_text = if vim.mode() == &VimMode::Command {
            vim.command_buffer()
        } else {
            ""
        };
        
        let command = Paragraph::new(command_text)
            .style(Style::default().fg(color_scheme.foreground_color()).bg(color_scheme.background_color()));
        
        f.render_widget(command, area);
    }
    
    /// Update scroll offset based on cursor position
    fn update_scroll(&mut self, buffer: &Buffer) {
        let (cursor_line, _) = buffer.cursor_position();
        
        if let Some(terminal) = &self.terminal {
            let terminal_height = terminal.size().unwrap_or_default().height as usize;
            let editor_height = terminal_height.saturating_sub(3); // Account for status and command lines
            
            // Scroll down if cursor is below visible area
            if cursor_line >= self.scroll_offset + editor_height {
                self.scroll_offset = cursor_line - editor_height + 1;
            }
            
            // Scroll up if cursor is above visible area
            if cursor_line < self.scroll_offset {
                self.scroll_offset = cursor_line;
            }
        }
    }
    
    /// Update status message
    fn update_status_message(&mut self, buffer: &Buffer, _vim: &VimPlugin) {
        // Clear status message after some time or update with current info
        self.status_message.clear();
        
        // Add any relevant status information
        if buffer.is_modified() {
            self.status_message = "Modified".to_string();
        }
    }
    
    /// Set a temporary status message
    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
    }
    
    /// Get terminal size
    pub fn terminal_size(&self) -> Option<(u16, u16)> {
        self.terminal.as_ref().and_then(|t| {
            t.size().ok().map(|rect| (rect.width, rect.height))
        })
    }
}

impl Plugin for TuiPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn initialize(&mut self) -> Result<()> {
        println!("Initializing TUI plugin");
        self.setup_terminal()?;
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<()> {
        println!("Shutting down TUI plugin");
        self.restore_terminal()?;
        Ok(())
    }
}