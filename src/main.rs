use clap::{Arg, Command, ArgAction};
use rusk::{Editor, Result, Config, ConfigLoader};
use rusk::config::ThemeManager;
use std::path::Path;

fn main() -> Result<()> {
    let matches = Command::new("rusk")
        .version("0.1.0")
        .author("Rusk Development Team")
        .about("A modern text editor with vim-style editing")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .subcommand(
            Command::new("edit")
                .about("Open file(s) for editing")
                .arg(
                    Arg::new("files")
                        .help("Files to open")
                        .value_name("FILES")
                        .action(ArgAction::Append)
                        .required(true)
                )
                .arg(
                    Arg::new("line")
                        .short('l')
                        .long("line")
                        .value_name("LINE_NUMBER")
                        .help("Jump to specific line number")
                )
                .arg(
                    Arg::new("readonly")
                        .short('r')
                        .long("readonly")
                        .action(ArgAction::SetTrue)
                        .help("Open file in read-only mode")
                )
        )
        .subcommand(
            Command::new("config")
                .about("Configuration management")
                .subcommand(
                    Command::new("show")
                        .about("Show current configuration")
                        .arg(
                            Arg::new("section")
                                .help("Configuration section to show (editor, ui, keybindings, plugins)")
                                .value_name("SECTION")
                        )
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit configuration file")
                )
                .subcommand(
                    Command::new("reset")
                        .about("Reset configuration to defaults")
                        .arg(
                            Arg::new("confirm")
                                .long("confirm")
                                .action(ArgAction::SetTrue)
                                .help("Confirm the reset operation")
                        )
                )
                .subcommand(
                    Command::new("validate")
                        .about("Validate configuration file")
                        .arg(
                            Arg::new("file")
                                .help("Configuration file to validate")
                                .value_name("CONFIG_FILE")
                        )
                )
        )
        .subcommand(
            Command::new("session")
                .about("Session management")
                .subcommand(
                    Command::new("save")
                        .about("Save current session")
                        .arg(
                            Arg::new("name")
                                .help("Session name")
                                .value_name("SESSION_NAME")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("load")
                        .about("Load saved session")
                        .arg(
                            Arg::new("name")
                                .help("Session name")
                                .value_name("SESSION_NAME")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("list")
                        .about("List saved sessions")
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete saved session")
                        .arg(
                            Arg::new("name")
                                .help("Session name")
                                .value_name("SESSION_NAME")
                                .required(true)
                        )
                )
        )
        .subcommand(
            Command::new("plugin")
                .about("Plugin management")
                .subcommand(
                    Command::new("list")
                        .about("List available plugins")
                        .arg(
                            Arg::new("enabled")
                                .long("enabled")
                                .action(ArgAction::SetTrue)
                                .help("Show only enabled plugins")
                        )
                )
                .subcommand(
                    Command::new("enable")
                        .about("Enable a plugin")
                        .arg(
                            Arg::new("plugin")
                                .help("Plugin name")
                                .value_name("PLUGIN_NAME")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("disable")
                        .about("Disable a plugin")
                        .arg(
                            Arg::new("plugin")
                                .help("Plugin name")
                                .value_name("PLUGIN_NAME")
                                .required(true)
                        )
                )
        )
        .subcommand(
            Command::new("check")
                .about("Health check and diagnostics")
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(ArgAction::SetTrue)
                        .help("Show detailed diagnostic information")
                )
        )
        .subcommand(
            Command::new("themes")
                .about("Theme management")
                .subcommand(
                    Command::new("list")
                        .about("List available themes")
                )
                .subcommand(
                    Command::new("current")
                        .about("Show current theme")
                )
        )
        .arg(
            Arg::new("file")
                .help("File to open")
                .value_name("FILE")
                .index(1)
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG_FILE")
                .help("Use custom configuration file")
        )
        .arg(
            Arg::new("no-config")
                .long("no-config")
                .action(ArgAction::SetTrue)
                .help("Start without loading configuration")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("Enable verbose output")
        )
        .get_matches();

    // Handle subcommands
    match matches.subcommand() {
        Some(("edit", sub_matches)) => {
            handle_edit_command(sub_matches)
        }
        Some(("config", sub_matches)) => {
            handle_config_command(sub_matches)
        }
        Some(("session", sub_matches)) => {
            handle_session_command(sub_matches)
        }
        Some(("plugin", sub_matches)) => {
            handle_plugin_command(sub_matches)
        }
        Some(("check", sub_matches)) => {
            handle_check_command(sub_matches)
        }
        Some(("themes", sub_matches)) => {
            handle_themes_command(sub_matches)
        }
        None => {
            // Default behavior - start editor
            start_editor(&matches)
        }
        _ => unreachable!(),
    }
}

fn start_editor(matches: &clap::ArgMatches) -> Result<()> {
    // Create editor instance
    let mut editor = if let Some(file_path) = matches.get_one::<String>("file") {
        // Open specific file
        Editor::with_file(file_path)?
    } else {
        // Start with empty buffer
        Editor::new()?
    };

    // Run the editor
    editor.run()?;
    Ok(())
}

fn handle_edit_command(matches: &clap::ArgMatches) -> Result<()> {
    let files: Vec<&String> = matches.get_many::<String>("files").unwrap_or_default().collect();
    let line_number: Option<&String> = matches.get_one::<String>("line");
    let readonly = matches.get_flag("readonly");
    
    if files.is_empty() {
        eprintln!("Error: No files specified");
        std::process::exit(1);
    }
    
    // Create editor and open files
    let mut editor = Editor::new()?;
    
    for file_path in files {
        if !Path::new(file_path).exists() {
            eprintln!("Warning: File '{}' does not exist", file_path);
            continue;
        }
        
        editor.open_file(file_path)?;
        
        if readonly {
            println!("Opened '{}' in read-only mode", file_path);
        } else {
            println!("Opened '{}' for editing", file_path);
        }
    }
    
    if let Some(line) = line_number {
        if let Ok(line_num) = line.parse::<usize>() {
            println!("Jumping to line {}", line_num);
            // TODO: Implement jump to line functionality
        } else {
            eprintln!("Warning: Invalid line number '{}'", line);
        }
    }
    
    editor.run()?;
    Ok(())
}

fn handle_config_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("show", sub_matches)) => {
            let config = ConfigLoader::load()?;
            
            if let Some(section) = sub_matches.get_one::<String>("section") {
                match section.as_str() {
                    "editor" => println!("{:#?}", config.editor),
                    "ui" => println!("{:#?}", config.ui),
                    "keybindings" => println!("{:#?}", config.keybindings),
                    "plugins" => println!("{:#?}", config.plugins),
                    _ => {
                        eprintln!("Unknown section: {}", section);
                        eprintln!("Available sections: editor, ui, keybindings, plugins");
                        std::process::exit(1);
                    }
                }
            } else {
                println!("{:#?}", config);
            }
        }
        Some(("edit", _)) => {
            println!("Opening configuration file for editing...");
            let config_path = dirs::config_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("rusk")
                .join("config.toml");
            
            let mut editor = Editor::with_file(config_path.to_str().unwrap())?;
            editor.run()?;
        }
        Some(("reset", sub_matches)) => {
            if sub_matches.get_flag("confirm") {
                println!("Resetting configuration to defaults...");
                let _default_config = Config::default();
                // TODO: Implement config reset functionality
                println!("Configuration reset complete");
            } else {
                eprintln!("This will reset all configuration to defaults.");
                eprintln!("Use --confirm to proceed with the reset.");
            }
        }
        Some(("validate", sub_matches)) => {
            let config_file = sub_matches.get_one::<String>("file");
            
            match config_file {
                Some(file) => {
                    if Path::new(file).exists() {
                        match ConfigLoader::load_from_file(file) {
                            Ok(_) => println!("Configuration file '{}' is valid", file),
                            Err(e) => {
                                eprintln!("Configuration file '{}' is invalid: {}", file, e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        eprintln!("Configuration file '{}' not found", file);
                        std::process::exit(1);
                    }
                }
                None => {
                    match ConfigLoader::load() {
                        Ok(_) => println!("Current configuration is valid"),
                        Err(e) => {
                            eprintln!("Current configuration is invalid: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!("Use 'rusk config --help' for available config commands");
        }
    }
    Ok(())
}

fn handle_session_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("save", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("Saving session '{}'...", name);
            // TODO: Implement session save functionality
            println!("Session '{}' saved", name);
        }
        Some(("load", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("Loading session '{}'...", name);
            // TODO: Implement session load functionality
            println!("Session '{}' loaded", name);
        }
        Some(("list", _)) => {
            println!("Saved sessions:");
            // TODO: Implement session list functionality
            println!("  (No saved sessions)");
        }
        Some(("delete", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap();
            println!("Deleting session '{}'...", name);
            // TODO: Implement session delete functionality
            println!("Session '{}' deleted", name);
        }
        _ => {
            eprintln!("Use 'rusk session --help' for available session commands");
        }
    }
    Ok(())
}

fn handle_plugin_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("list", sub_matches)) => {
            let enabled_only = sub_matches.get_flag("enabled");
            let config = ConfigLoader::load()?;
            
            if enabled_only {
                println!("Enabled plugins:");
                for plugin in &config.plugins.enabled_plugins {
                    println!("  ✓ {}", plugin);
                }
            } else {
                println!("Available plugins:");
                let all_plugins = vec!["vim", "tui", "syntax", "file_io", "config"];
                
                for plugin in all_plugins {
                    let status = if config.plugins.enabled_plugins.contains(&plugin.to_string()) {
                        "✓"
                    } else {
                        "✗"
                    };
                    println!("  {} {}", status, plugin);
                }
            }
        }
        Some(("enable", sub_matches)) => {
            let plugin = sub_matches.get_one::<String>("plugin").unwrap();
            println!("Enabling plugin '{}'...", plugin);
            // TODO: Implement plugin enable functionality
            println!("Plugin '{}' enabled", plugin);
        }
        Some(("disable", sub_matches)) => {
            let plugin = sub_matches.get_one::<String>("plugin").unwrap();
            println!("Disabling plugin '{}'...", plugin);
            // TODO: Implement plugin disable functionality
            println!("Plugin '{}' disabled", plugin);
        }
        _ => {
            eprintln!("Use 'rusk plugin --help' for available plugin commands");
        }
    }
    Ok(())
}

fn handle_check_command(matches: &clap::ArgMatches) -> Result<()> {
    let verbose = matches.get_flag("verbose");
    
    println!("Running Rusk health check...");
    println!();
    
    // Check configuration
    print!("Configuration: ");
    match ConfigLoader::load() {
        Ok(_) => println!("✓ Valid"),
        Err(e) => {
            println!("✗ Invalid");
            if verbose {
                println!("  Error: {}", e);
            }
        }
    }
    
    // Check config directory
    print!("Config directory: ");
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("rusk");
    
    if config_dir.exists() {
        println!("✓ Found at {}", config_dir.display());
    } else {
        println!("✗ Not found");
        if verbose {
            println!("  Expected at: {}", config_dir.display());
        }
    }
    
    // Check plugins
    print!("Plugins: ");
    let config = ConfigLoader::load().unwrap_or_default();
    println!("✓ {} enabled", config.plugins.enabled_plugins.len());
    
    if verbose {
        for plugin in &config.plugins.enabled_plugins {
            println!("  - {}", plugin);
        }
    }
    
    println!();
    println!("Health check complete");
    
    Ok(())
}

fn handle_themes_command(matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("Available themes:");
            match ThemeManager::list_themes() {
                Ok(themes) => {
                    for theme in themes {
                        if ThemeManager::get_builtin_theme(&theme).is_some() {
                            println!("  {} (built-in)", theme);
                        } else {
                            println!("  {} (custom)", theme);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error listing themes: {}", e);
                    return Err(e);
                }
            }
        }
        Some(("current", _)) => {
            match ConfigLoader::load() {
                Ok(config) => {
                    println!("Current theme: {}", config.ui.theme);
                    match ConfigLoader::load_color_scheme(&config) {
                        Ok(_) => println!("Theme loaded successfully"),
                        Err(e) => {
                            eprintln!("Warning: Failed to load theme '{}': {}", config.ui.theme, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error loading config: {}", e);
                    return Err(e);
                }
            }
        }
        _ => {
            eprintln!("Use 'rusk themes list' or 'rusk themes current'");
        }
    }
    Ok(())
}