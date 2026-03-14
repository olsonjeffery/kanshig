#![deny(warnings)]

use clap::Parser;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui_textarea::TextArea;
use std::cell::RefCell;
use std::fs;
use std::io;
use std::rc::Rc;

use ratatui::crossterm::{event, execute};
use ratatui::{Terminal, layout::Rect, prelude::CrosstermBackend};

use crate::input::update_input;
use crate::tui::build_output_details;
use crate::tui::build_profile_details;
use crate::tui::normalize_index;

mod input;
mod model;
mod niri;
mod parser;
mod tui;
mod validation;

pub const DO_EDITOR_FLOW: &str = "DO_EDITOR_FLOW";

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

    /// Output unified outputs as JSON and exit (skip TUI)
    #[arg(long)]
    json: bool,
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
    let mut content = "".to_owned();

    if path.exists() {
        log::info!("Config file found at: {}", config_path);

        // Load the file as a string
        match fs::read_to_string(&config_path) {
            Ok(c_content) => {
                log::info!("Config file content loaded successfully");

                // Validate the config
                match validation::validate_config(&c_content) {
                    Ok(_) => {
                        log::info!("Config validation passed");

                        // Parse into data model structs
                        match parser::parse_config(&c_content) {
                            Ok((parsed_config, c_content)) => {
                                log::info!("Config parsed into data model structs");
                                config = Some(parsed_config);
                                content = c_content;
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

    // Handle JSON output mode (skip TUI)
    if args.json {
        let unified_outputs = build_unified_outputs(config.as_ref(), niri_outputs.as_ref());
        let json_output = serde_json::to_string_pretty(&unified_outputs)?;
        println!("{}", json_output);
        return Ok(());
    }

    // Initialize TUI with mouse support
    let mut terminal = ratatui::init();

    // Enable mouse support
    execute!(terminal.backend_mut(), event::EnableMouseCapture)?;

    // Run the TUI
    run_tui(
        &mut terminal,
        config.as_ref(),
        niri_outputs.as_ref(),
        &config_path,
        Some(content),
    )?;

    // Disable mouse support before exiting
    execute!(terminal.backend_mut(), event::DisableMouseCapture)?;

    ratatui::restore();

    Ok(())
}

/// Build a list of unified outputs from config and niri outputs
fn build_unified_outputs(
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
) -> Vec<crate::model::UnifiedOutput> {
    let mut unified_outputs: Vec<crate::model::UnifiedOutput> = Vec::new();

    // Start with config outputs if available
    if let Some(cfg) = config {
        for output in &cfg.outputs {
            unified_outputs.push(crate::model::UnifiedOutput::from_config(output.clone()));
        }
    }

    // Mark outputs as detected and add detected-only outputs
    if let Some(niri) = niri_outputs {
        for niri_output in niri.values() {
            let mut match_found = false;
            let ni_model = niri_output.model.as_deref().unwrap_or("");

            // Try to match with existing configured outputs
            for unified in &mut unified_outputs {
                if unified.name.contains(ni_model) {
                    unified.mark_detected();
                    unified.niri_output = Some(niri_output.clone());
                    match_found = true;
                    break;
                }
            }

            // If no match found, add as detected-only output
            if !match_found {
                let mode = if !niri_output.modes.is_empty() {
                    let preferred_mode = niri_output
                        .modes
                        .iter()
                        .find(|m| m.is_preferred)
                        .unwrap_or(&niri_output.modes[0]);
                    format!(
                        "{}x{}@{:.3}",
                        preferred_mode.width,
                        preferred_mode.height,
                        preferred_mode.refresh_rate as f64 / 1000.0
                    )
                } else {
                    String::new()
                };

                unified_outputs.push(crate::model::UnifiedOutput {
                    name: format!("display: {} (model {})", niri_output.name, ni_model),
                    mode,
                    position: String::new(),
                    scale: 1.0,
                    alias: None,
                    detected: true,
                    configured: false,
                    niri_output: Some(niri_output.clone()),
                });
            }
        }
    }

    unified_outputs
}

#[derive(Debug, Clone)]
pub enum KanshigTuiState<'a> {
    OutputsFocused(i32, i32),
    ProfilesFocused(i32, i32),
    HelpPopup,
    EditConfig {
        textarea: Rc<RefCell<TextArea<'a>>>,
        original_content: String,
    },
    AddOutputPopup {
        add_output_state: crate::model::AddOutputWindowState<'a>,
        previous_outputs_index: i32,
    },
    QuitNow,
}

pub const WRITE_CONFIG: event::KeyCode = event::KeyCode::Char('W');

fn run_tui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    config: Option<&crate::model::KanshiConfig>,
    niri_outputs: Option<&crate::model::NiriOutputs>,
    config_path: &str,
    config_content: Option<String>,
) -> io::Result<()> {
    let mut selected = KanshigTuiState::OutputsFocused(0, 0);

    // Build unified outputs for input handling
    let unified_outputs = build_unified_outputs(config, niri_outputs);

    // Create a loop to handle events
    loop {
        // Draw the UI
        let new_selected = selected.clone();
        terminal.draw(|frame| {
            draw_ui(frame, config, niri_outputs, &new_selected);
        })?;

        // Check for events
        if event::poll(std::time::Duration::from_millis(100))?
            && let event::Event::Key(key) = event::read()?
        {
            // Get the selected unified output if outputs are focused
            let selected_unified_output = match &selected {
                KanshigTuiState::OutputsFocused(oi, _) => {
                    let list_len = unified_outputs.len();
                    if list_len > 0 {
                        let selected_idx = tui::normalize_index(*oi, list_len);
                        unified_outputs.get(selected_idx)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            let new_selected = selected.clone();
            let new_selected = update_input(
                new_selected,
                key,
                config_content.as_ref(),
                config_path,
                niri_outputs,
                selected_unified_output,
            );
            if let KanshigTuiState::QuitNow = new_selected {
                break;
            }
            selected = new_selected;
        }
    }

    Ok(())
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

    // Draw add output popup if active
    if let KanshigTuiState::AddOutputPopup {
        add_output_state, ..
    } = selected
    {
        draw_add_output_popup(frame, area, add_output_state);
        return;
    }

    // Split the area into sections
    let chunks = ratatui::layout::Layout::horizontal([
        ratatui::layout::Constraint::Percentage(40),
        ratatui::layout::Constraint::Percentage(60),
    ])
    .split(area);

    let picker_lists = chunks[0];
    let details_area = chunks[1];
    //let outputs_chunk = chunks[0];
    //let profiles_chunk = chunks[1];
    let picker_chunks = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Percentage(50),
        ratatui::layout::Constraint::Percentage(50),
    ])
    .split(picker_lists);
    let outputs_chunk = picker_chunks[0];
    let profiles_chunk = picker_chunks[1];

    // Draw unified outputs if both config and niri_outputs are available
    if let Some(c) = config {
        if let Some(outputs) = niri_outputs {
            // only unified outputs in its <25% square
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
        // list only
        tui::display_profiles(frame, config, profiles_chunk, selected);
    }

    //let text = build_output_details(output)
    let def_item = model::KanshiConfig::default();
    let config = config.unwrap_or(&def_item);
    let details_text = match selected {
        KanshigTuiState::OutputsFocused(oi, _) => {
            let unified_outputs = build_unified_outputs(Some(config), niri_outputs);
            if !unified_outputs.is_empty() {
                let selected_idx = normalize_index(*oi, unified_outputs.len());
                if selected_idx < unified_outputs.len() {
                    build_output_details(&unified_outputs[selected_idx])
                } else {
                    "No output selected".to_string()
                }
            } else {
                "No outputs available".to_string()
            }
        }
        KanshigTuiState::ProfilesFocused(pi, _) => {
            let profiles_len = config.profiles.len();
            if profiles_len > 0 {
                let selected_idx = normalize_index(*pi, profiles_len);
                let profile = config
                    .profiles
                    .get(selected_idx)
                    .expect("main.rs profile focused expected some profile, got none");
                if selected_idx < profiles_len {
                    build_profile_details(profile, niri_outputs)
                } else {
                    "No output selected".to_string()
                }
            } else {
                "No outputs available".to_string()
            }
        }
        KanshigTuiState::EditConfig { .. } => {
            // ?
            //textarea
            DO_EDITOR_FLOW.to_owned()
        }
        _ => "Select an output to view details".to_string(),
    };

    if details_text == DO_EDITOR_FLOW {
        match selected {
            KanshigTuiState::EditConfig { textarea, .. } => {
                frame.render_widget(&*textarea.borrow(), details_area);
            }
            d => panic!(
                "expected an EditConfig to be current input state, but was instead {:?}",
                d
            ),
        }
    } else {
        let details_widget = ratatui::widgets::Paragraph::new(details_text)
            .block(Block::new().title("Output Details").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));
        frame.render_widget(details_widget, details_area);
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
            keys: vec!["q", "Esc"],
            description: "Quit",
        },
        HelpEntry {
            keys: vec!["?"],
            description: "Toggle help popup",
        },
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
            keys: vec!["e", "Open Kanshi Config Editor"],
            description: "Quit",
        },
        HelpEntry {
            keys: vec!["CTRL+d, Esc", "Editor: Discard"],
            description: "Quit",
        },
        HelpEntry {
            keys: vec!["CTRL+s", "Editor: Save"],
            description: "Quit",
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

/// Draw the add output popup centered on screen
fn draw_add_output_popup(
    frame: &mut ratatui::Frame,
    area: Rect,
    add_output_state: &crate::model::AddOutputWindowState,
) {
    // Calculate popup dimensions
    let popup_width = 60;
    let popup_height = 18; // Title + 4 fields + 2 buttons + padding
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;
    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Create the popup block
    let popup_block = ratatui::widgets::Block::new()
        .title(" Add Output to Config ")
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));

    // Layout for popup content
    let chunks = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Length(3), // Output name label
        ratatui::layout::Constraint::Length(2), // Mode
        ratatui::layout::Constraint::Length(2), // Position
        ratatui::layout::Constraint::Length(2), // Scale
        ratatui::layout::Constraint::Length(2), // Alias
        ratatui::layout::Constraint::Length(3), // Buttons
    ])
    .split(popup_area);

    // Output name label
    let name_label = ratatui::widgets::Paragraph::new(
        ratatui::text::Line::raw(format!("Output: {}", add_output_state.output_name))
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)),
    );
    frame.render_widget(name_label, chunks[0]);

    // Helper function to render a field with label and textarea
    let _is_focused = |focus: crate::model::AddOutputFocus| add_output_state.focus == focus;

    // Mode field
    let mode_label = ratatui::widgets::Paragraph::new("Mode:");
    frame.render_widget(mode_label, chunks[1]);
    frame.render_widget(&*add_output_state.mode.borrow(), chunks[1]);

    // Position field
    let pos_label = ratatui::widgets::Paragraph::new("Position:");
    frame.render_widget(pos_label, chunks[2]);
    frame.render_widget(&*add_output_state.position.borrow(), chunks[2]);

    // Scale field
    let scale_label = ratatui::widgets::Paragraph::new("Scale:");
    frame.render_widget(scale_label, chunks[3]);
    frame.render_widget(&*add_output_state.scale.borrow(), chunks[3]);

    // Alias field
    let alias_label = ratatui::widgets::Paragraph::new("Alias:");
    frame.render_widget(alias_label, chunks[4]);
    frame.render_widget(&*add_output_state.alias.borrow(), chunks[4]);

    // Buttons
    let buttons_text = if _is_focused(crate::model::AddOutputFocus::AddButton) {
        "[>] Add    Cancel".to_string()
    } else if _is_focused(crate::model::AddOutputFocus::CancelButton) {
        "Add    [>] Cancel".to_string()
    } else {
        "[ ] Add    [ ] Cancel".to_string()
    };

    let buttons = ratatui::widgets::Paragraph::new(buttons_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::new()
                .title(" Press Enter to confirm, Esc to cancel ")
                .borders(Borders::NONE),
        );
    frame.render_widget(buttons, chunks[5]);

    // Render the popup block
    frame.render_widget(popup_block, popup_area);
}
