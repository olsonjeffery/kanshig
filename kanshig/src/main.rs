#![deny(warnings)]

use clap::Parser;
use std::fs;
use std::io;

use ratatui::crossterm::{event, execute};
use ratatui::{Terminal, layout::Rect, prelude::CrosstermBackend};

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

    log::info!("kanshig CLI initialized successfully");

    // Initialize TUI with mouse support
    let mut terminal = ratatui::init();

    // Enable mouse support
    execute!(terminal.backend_mut(), event::EnableMouseCapture)?;

    // Run the TUI
    run_tui(&mut terminal, config.as_ref(), niri_outputs.as_ref())?;

    // Disable mouse support before exiting
    execute!(terminal.backend_mut(), event::DisableMouseCapture)?;

    ratatui::restore();

    Ok(())
}

#[derive(Copy, Clone, Debug)]
pub enum KanshigTuiState {
    OutputsFocused(i32, i32),
    ProfilesFocused(i32, i32),
    HelpPopup,
    QuitNow,
}

pub const MOVE_SET: &[event::KeyCode] = &[
    event::KeyCode::Up,
    event::KeyCode::Down,
    event::KeyCode::Left,
    event::KeyCode::Right,
    event::KeyCode::Char('w'),
    event::KeyCode::Char('a'),
    event::KeyCode::Char('s'),
    event::KeyCode::Char('d'),
    event::KeyCode::Char('j'),
    event::KeyCode::Char('k'),
    event::KeyCode::Tab,
];

pub const WRITE_CONFIG: event::KeyCode = event::KeyCode::Char('W');

fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
) -> io::Result<()> {
    let mut selected = KanshigTuiState::OutputsFocused(0, 0);
    // Create a loop to handle events
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            draw_ui(frame, config, niri_outputs, &selected);
        })?;

        // Check for events
        if event::poll(std::time::Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()?
        {
            selected = update_input(&selected, key);
            if let KanshigTuiState::QuitNow = selected {
                break;
            }
        }
    }

    Ok(())
}

fn update_input(in_selected: &KanshigTuiState, key: event::KeyEvent) -> KanshigTuiState {
    let selected = *in_selected;

    // Handle help popup toggle with '?'
    if key.code == event::KeyCode::Char('?') {
        return match selected {
            KanshigTuiState::OutputsFocused(_, _) | KanshigTuiState::ProfilesFocused(_, _) => {
                KanshigTuiState::HelpPopup
            }
            KanshigTuiState::HelpPopup => KanshigTuiState::OutputsFocused(0, 0),
            _ => selected,
        };
    }

    // When help popup is open, any key closes it
    if let KanshigTuiState::HelpPopup = selected {
        return KanshigTuiState::OutputsFocused(0, 0);
    }

    // Exit on 'q' or Escape
    if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc {
        return KanshigTuiState::QuitNow;
    }
    if MOVE_SET.contains(&key.code) {
        match selected {
            KanshigTuiState::QuitNow => KanshigTuiState::QuitNow,
            KanshigTuiState::OutputsFocused(oi, pi) => {
                if let event::KeyCode::Tab = key.code {
                    KanshigTuiState::ProfilesFocused(pi, oi)
                } else if let event::KeyCode::Up
                | event::KeyCode::Char('k')
                | event::KeyCode::Char('w') = key.code
                {
                    // UP
                    let new_val = oi - 1;
                    KanshigTuiState::OutputsFocused(new_val, pi)
                } else if let event::KeyCode::Down
                | event::KeyCode::Char('j')
                | event::KeyCode::Char('s') = key.code
                {
                    //Down
                    let new_val = oi + 1;
                    KanshigTuiState::OutputsFocused(new_val, pi)
                } else {
                    selected
                }
            }
            KanshigTuiState::ProfilesFocused(oi, pi) => {
                if let event::KeyCode::Tab = key.code {
                    KanshigTuiState::OutputsFocused(pi, oi)
                } else if let event::KeyCode::Up
                | event::KeyCode::Char('k')
                | event::KeyCode::Char('w') = key.code
                {
                    // UP
                    let new_val = oi - 1;
                    KanshigTuiState::ProfilesFocused(new_val, pi)
                } else if let event::KeyCode::Down
                | event::KeyCode::Char('j')
                | event::KeyCode::Char('s') = key.code
                {
                    //Down
                    let new_val = oi + 1;
                    KanshigTuiState::ProfilesFocused(new_val, pi)
                } else {
                    selected
                }
            }
            KanshigTuiState::HelpPopup => selected,
        }
    } else {
        selected
    }
}

fn draw_ui(
    frame: &mut ratatui::Frame,
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
    selected: &KanshigTuiState,
) {
    let area = frame.area();

    // Draw help popup if active
    if let KanshigTuiState::HelpPopup = selected {
        draw_help_popup(frame, area);
        return;
    }

    // Split the area into sections
    let chunks = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Percentage(50),
        ratatui::layout::Constraint::Percentage(50),
    ])
    .split(area);

    let outputs_chunk = chunks[0];
    let profiles_chunk = chunks[1];

    // Draw unified outputs if both config and niri_outputs are available
    if let Some(c) = config {
        if let Some(outputs) = niri_outputs {
            tui::display_unified_outputs(frame, c, outputs, outputs_chunk, selected);
        } else {
            // Fallback to just config display
            tui::display_config(frame, c, outputs_chunk);
        }
    } else if let Some(outputs) = niri_outputs {
        // Fallback to just niri outputs display
        tui::display_niri_outputs(frame, outputs, outputs_chunk);
    } else {
        frame.render_widget(
            ratatui::widgets::Paragraph::new("No Kanshi config or Niri outputs found").block(
                ratatui::widgets::Block::new()
                    .title("Status")
                    .borders(ratatui::widgets::Borders::ALL),
            ),
            outputs_chunk,
        );
    }

    let has_profiles = config.is_some();
    if has_profiles {
        tui::display_profiles(frame, config, niri_outputs, profiles_chunk, selected);
    }
}

/// Represents a help entry with keys and description
#[derive(Debug, Clone)]
struct HelpEntry {
    keys: Vec<&'static str>,
    description: &'static str,
}

/// Draw the help popup centered on screen
fn draw_help_popup(frame: &mut ratatui::Frame, area: Rect) {
    // Define help entries - easily extensible for future additions
    let help_entries = vec![
        HelpEntry {
            keys: vec!["j", "s", "↓"],
            description: "Move down",
        },
        HelpEntry {
            keys: vec!["k", "w", "↑"],
            description: "Move up",
        },
        HelpEntry {
            keys: vec!["Tab"],
            description: "Switch focus between panels",
        },
        HelpEntry {
            keys: vec!["q", "Esc"],
            description: "Quit",
        },
        HelpEntry {
            keys: vec!["?"],
            description: "Toggle help popup",
        },
    ];

    // Calculate popup dimensions
    let popup_width = 50;
    let popup_height = help_entries.len() + 4; // Title + entries + footer + padding
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height as u16)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height as u16);

    // Create the popup block
    let popup_block = ratatui::widgets::Block::new()
        .title(" Help (press any key to close) ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow));

    // Build the help content
    let mut lines: Vec<ratatui::text::Line> = Vec::new();
    for entry in help_entries {
        let keys_text: String = entry.keys.join(", ");
        let line = format!("{:<15} {}", keys_text, entry.description);
        lines.push(ratatui::text::Line::raw(line));
    }

    let help_text = ratatui::text::Text::from(lines);
    let help_paragraph = ratatui::widgets::Paragraph::new(help_text)
        .block(popup_block)
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

    // Render the popup
    frame.render_widget(help_paragraph, popup_area);
}
