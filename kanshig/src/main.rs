#![deny(warnings)]

use clap::Parser;
use std::fs;
use std::io;

use ratatui::crossterm::{event, execute};
use ratatui::{Terminal, prelude::CrosstermBackend};

mod model;
mod niri;
mod parser;
mod tui;
mod validation;

/// kanshig - A TUI application for generating and updating Kanshi configs
#[derive(Parser, Debug)]
#[command(name = "kanshig")]
#[command(author = "jeff")]
#[command(version = "0.1.0")]
#[command(about = "Generate and update Kanshi configs from window manager state", long_about = None)]
struct Args {
    /// Load the kanshi config from a custom location
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> io::Result<()> {
    // Initialize logging
    env_logger::init();

    let args = Args::parse();

    // Determine the config path to load
    let config_path = if let Some(path) = &args.config {
        path.clone()
    } else {
        // Default kanshi config location
        format!(
            "{}/.config/kanshi/config",
            std::env::var("HOME").unwrap_or_else(|_| String::from("/"))
        )
    };

    log::info!("Loading kanshi config from: {}", config_path);

    // Check if the file exists and load it
    let path = std::path::Path::new(&config_path);
    let mut config: Option<crate::model::KanshiConfig> = None;

    if path.exists() {
        log::info!("Config file found at: {}", config_path);

        // Load the file as a string
        match fs::read_to_string(&config_path) {
            Ok(content) => {
                log::info!("Config file content loaded successfully");

                // Validate the config
                match validation::validate_config(&content) {
                    Ok(_) => {
                        log::info!("Config validation passed");

                        // Parse into data model structs
                        match parser::parse_config(&content) {
                            Ok(parsed_config) => {
                                log::info!("Config parsed into data model structs");
                                config = Some(parsed_config);
                            }
                            Err(e) => {
                                log::error!(
                                    "Failed to parse config into data model structs: {}",
                                    e
                                );
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Config validation failed: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to read config file: {}", e);
            }
        }
    } else {
        log::warn!("Config file not found: {}", config_path);
    }

    // Call niri msg --json outputs and display the results
    let niri_outputs = match niri::get_niri_outputs() {
        Ok(outputs) => {
            log::info!("Successfully retrieved {} niri outputs:", outputs.len());
            for (name, output) in outputs.iter() {
                log::info!(
                    "  - {}: {} {}",
                    name,
                    output.make.as_ref().unwrap_or(&String::new()),
                    output.model.as_ref().unwrap_or(&String::new())
                );
            }
            Some(outputs)
        }
        Err(e) => {
            log::warn!("Failed to retrieve niri outputs: {}", e);
            None
        }
    };

    // Display the config and niri outputs using TUI
    if let Some(ref c) = config {
        log::info!("Parsed Kanshi Config:");
        log::info!("  Outputs: {}", c.outputs.len());
        for output in &c.outputs {
            log::info!(
                "    - {}: {} (scale: {}, position: {})",
                output.name,
                output.mode,
                output.scale,
                output.position
            );
            if let Some(alias) = &output.alias {
                log::info!("      Alias: {}", alias);
            }
        }

        log::info!("  Profiles: {}", c.profiles.len());
        for profile in &c.profiles {
            log::info!("    - {}: {} outputs", profile.name, profile.outputs.len());
            for output in &profile.outputs {
                let status = if output.enabled {
                    "enabled"
                } else {
                    "disabled"
                };
                log::info!("      - {} {}", output.alias, status);
            }
        }
    }

    if let Some(ref outputs) = niri_outputs {
        log::info!("Niri Outputs: {} items", outputs.len());
        for (name, output) in outputs.iter() {
            log::info!(
                "  - {}: {} {}",
                name,
                output.make.as_ref().unwrap_or(&String::new()),
                output.model.as_ref().unwrap_or(&String::new())
            );
        }
    }

    log::info!("kanshig CLI initialized successfully");

    // Initialize TUI with mouse support
    let backend = CrosstermBackend::new(std::io::stderr());
    let mut terminal = Terminal::new(backend)?;

    // Enable mouse support
    execute!(terminal.backend_mut(), event::EnableMouseCapture)?;

    // Run the TUI
    run_tui(&mut terminal, config.as_ref(), niri_outputs.as_ref())?;

    // Disable mouse support before exiting
    execute!(terminal.backend_mut(), event::DisableMouseCapture)?;

    Ok(())
}

fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stderr>>,
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
) -> io::Result<()> {
    // Create a loop to handle events
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            draw_ui(frame, config, niri_outputs);
        })?;

        // Check for events
        if event::poll(std::time::Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()?
        {
            // Exit on 'q' or Escape
            if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc {
                break;
            }
        }
    }

    Ok(())
}

fn draw_ui(
    frame: &mut ratatui::Frame,
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
) {
    let area = frame.size();

    // Split the area into two sections
    let chunks = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Percentage(50),
        ratatui::layout::Constraint::Percentage(50),
    ])
    .split(area);

    // Draw config if available
    if let Some(c) = config {
        tui::display_config(frame, c);
    } else {
        frame.render_widget(
            ratatui::widgets::Paragraph::new("No Kanshi config found").block(
                ratatui::widgets::Block::new()
                    .title("Kanshi Config")
                    .borders(ratatui::widgets::Borders::ALL),
            ),
            chunks[0],
        );
    }

    // Draw niri outputs if available
    if let Some(outputs) = niri_outputs {
        tui::display_niri_outputs(frame, outputs);
    } else {
        frame.render_widget(
            ratatui::widgets::Paragraph::new("No Niri outputs found").block(
                ratatui::widgets::Block::new()
                    .title("Niri Outputs")
                    .borders(ratatui::widgets::Borders::ALL),
            ),
            chunks[1],
        );
    }
}
