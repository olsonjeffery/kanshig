//! TUI module for displaying kanshi config data

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::{
    KanshigTuiState,
    model::{self, KanshiConfig, NiriOutputs, UnifiedOutput},
};

/// Display the kanshi config outputs and profiles
#[allow(dead_code)]
pub fn display_config(f: &mut Frame, config: &KanshiConfig, chunk_rect: Rect) {
    let area = chunk_rect;

    // Create two columns for outputs and profiles
    let chunks =
        Layout::vertical([Constraint::Percentage(51), Constraint::Percentage(50)]).split(area);

    // Display outputs
    let outputs_list: Vec<ListItem> = config
        .outputs
        .iter()
        .map(|output| {
            let text = format!(
                "{} - {} (scale: {}, position: {})",
                output.name, output.mode, output.scale, output.position
            );
            ListItem::new(text)
        })
        .collect();

    let outputs_list_widget = List::new(outputs_list)
        .block(Block::new().title("Outputs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    f.render_widget(outputs_list_widget, chunks[0]);
}

/// Display the niri outputs
#[allow(dead_code)]
pub fn display_niri_outputs(f: &mut Frame, outputs: &NiriOutputs, display_chunk: Rect) {
    let area = display_chunk;

    // Create a simple list of outputs
    let mut outputs_list: Vec<ListItem> = Vec::new();
    for (name, output) in outputs.iter() {
        let make = output.make.clone().unwrap_or_default();
        let model = output.model.clone().unwrap_or_default();
        outputs_list.push(ListItem::new(format!("{} {} {}", name, make, model)));
    }

    let outputs_list_widget = List::new(outputs_list)
        .block(Block::new().title("Niri Outputs").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green));

    f.render_widget(outputs_list_widget, area);
}

pub fn display_profiles(
    f: &mut Frame,
    config: Option<&model::KanshiConfig>,
    niri_outputs: Option<&NiriOutputs>,
    profiles_chunk: ratatui::prelude::Rect,
    selected: &KanshigTuiState,
) {
    let config = match config {
        Some(t) => t.clone(),
        None => KanshiConfig::default(),
    };

    // Split the profiles area horizontally into list (left) and details (right)
    let profile_chunks =
        Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(profiles_chunk);

    let profiles_list_area = profile_chunks[0];
    let profiles_details_area = profile_chunks[1];

    // Build the profiles list (left side)
    let mut profiles_list = Vec::new();
    let list_len = config.profiles.len();
    for (idx, profile) in config.profiles.iter().enumerate() {
        let is_selected = match selected {
            KanshigTuiState::ProfilesFocused(pi, _) => modulo_match(*pi, idx as i32, list_len),
            _ => false,
        };
        let style = if is_selected {
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::White)
        } else {
            Style::default().fg(Color::White)
        };
        let output_count = profile.outputs.len();
        let ret_val =
            ListItem::new(format!("{} ({} outputs)", profile.name, output_count)).style(style);
        profiles_list.push(ret_val);
    }
    let box_style = if let KanshigTuiState::ProfilesFocused(_, _) = selected {
        Style::default().fg(Color::White).bold()
    } else {
        Style::default().fg(Color::Magenta)
    };
    let profiles_list_widget = List::new(profiles_list)
        .block(Block::new().title("Profiles").borders(Borders::ALL))
        .style(box_style);
    f.render_widget(profiles_list_widget, profiles_list_area);

    // Build the profile details (right side)
    let details_text = match selected {
        KanshigTuiState::ProfilesFocused(pi, _) => {
            if list_len > 0 {
                let selected_idx = normalize_index(*pi, list_len);
                if selected_idx < config.profiles.len() {
                    let profile = &config.profiles[selected_idx];
                    build_profile_details(profile, niri_outputs)
                } else {
                    "No profile selected".to_string()
                }
            } else {
                "No profiles available".to_string()
            }
        }
        _ => "Select a profile to view details".to_string(),
    };

    let details_widget = ratatui::widgets::Paragraph::new(details_text)
        .block(Block::new().title("Profile Details").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    f.render_widget(details_widget, profiles_details_area);
}

/// Normalize a potentially negative or out-of-bounds index to a valid index
fn normalize_index(idx: i32, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    let len_i32 = len as i32;
    let normalized = ((idx % len_i32) + len_i32) % len_i32;
    normalized as usize
}

/// Build the details text for a profile
fn build_profile_details(profile: &model::Profile, niri_outputs: Option<&NiriOutputs>) -> String {
    let mut lines = Vec::new();
    lines.push(format!("Profile: {}", profile.name));
    lines.push("".to_string());
    lines.push("Outputs:".to_string());

    for assignment in &profile.outputs {
        let status = if assignment.enabled {
            "enable"
        } else {
            "disable"
        };
        let mut detail_line = format!("  - {}: {}", assignment.alias, status);

        // Check if this output is detected in niri outputs
        if let Some(niri) = niri_outputs
            && is_output_detected(assignment.alias.as_str(), niri)
        {
            detail_line.push_str(" [DETECTED]");
        }
        lines.push(detail_line);
    }

    lines.join("\n")
}

/// Check if an output alias corresponds to a detected niri output
fn is_output_detected(alias: &str, niri_outputs: &NiriOutputs) -> bool {
    // Extract the display name from the alias (e.g., $HOME_0 -> HOME_0)
    let alias_name = alias.trim_start_matches('$');

    for (_, niri_output) in niri_outputs.iter() {
        let model = niri_output.model.as_deref().unwrap_or("");
        let name = niri_output.name.as_str();

        // Check if the alias matches the model or name
        if alias_name == model || alias_name == name {
            return true;
        }
        // Also check if the model/name contains the alias
        if model.contains(alias_name) || name.contains(alias_name) {
            return true;
        }
    }
    false
}

/// Display the unified outputs (combined config and niri outputs)
pub fn display_unified_outputs(
    f: &mut Frame,
    config: &KanshiConfig,
    niri_outputs: &NiriOutputs,
    display_chunk: Rect,
    selected: &KanshigTuiState,
) {
    let area = display_chunk;

    // Create a list of unified outputs
    let mut outputs_list: Vec<ListItem> = Vec::new();

    // First, create unified outputs from config
    let mut unified_outputs: Vec<UnifiedOutput> = config
        .outputs
        .iter()
        .map(|output| UnifiedOutput::from_config(output.clone()))
        .collect();

    // Mark outputs as detected if they exist in niri outputs
    for niri_item in niri_outputs {
        let mut match_found = false;
        for (idx, config_item) in &mut unified_outputs.clone().iter().enumerate() {
            if match_found {
                continue;
            }
            let ni_model = match niri_item.1.model.as_ref() {
                Some(s) => s.to_owned(),
                None => "does not match xxx23423".to_owned(),
            };
            let config_name = &config_item.name;
            //println!("comparing {} nad {}", config_name, ni_model);
            if config_name.contains(&ni_model) {
                let config_item = unified_outputs.get_mut(idx).unwrap();
                config_item.mark_detected();
                match_found = true;
            }
        }
        if !match_found {
            unified_outputs.push(UnifiedOutput {
                detected: true,
                configured: false,
                name: format!(
                    "display: {} (model {})",
                    niri_item.1.name.to_owned(),
                    niri_item.1.model.as_ref().unwrap(),
                ),
                alias: None,
                scale: 1.0,
                position: "".to_owned(),
                mode: if !niri_item.1.modes.is_empty() {
                    let mode = niri_item.1.modes[niri_item.1.modes.len() - 1].to_owned();
                    format!(
                        "{}x{} ({}hz) {}",
                        mode.width,
                        mode.height,
                        mode.refresh_rate,
                        if mode.is_preferred { "(PREFERRED)" } else { "" }
                    )
                } else {
                    "".to_owned()
                },
            })
        }
    }

    // Now create the list items with labels
    let list_len = unified_outputs.len();
    for (idx, unified) in unified_outputs.iter().enumerate() {
        let is_selected = match selected {
            KanshigTuiState::OutputsFocused(oi, _) => modulo_match(*oi, idx as i32, list_len),
            _ => false,
        };
        let mut labels = Vec::new();

        if unified.is_configured() {
            labels.push("CONFIGURED");
        }

        if unified.is_detected() {
            labels.push("DETECTED");
        }

        let labels_text = if labels.is_empty() {
            String::new()
        } else {
            format!(" [{}] ", labels.join(", "))
        };

        let text = format!("{}{}", labels_text, unified.name);

        // Choose color based on flags
        let style = if unified.is_detected() && unified.is_configured() {
            Style::default().fg(Color::Yellow)
        } else if unified.is_detected() {
            Style::default().fg(Color::Green)
        } else if unified.is_configured() {
            Style::default().fg(Color::White)
        } else {
            Style::default()
        };
        let style = if is_selected {
            style
                .add_modifier(Modifier::BOLD)
                .fg(Color::Black)
                .bg(Color::White)
        } else {
            style
        };

        outputs_list.push(ListItem::new(text).style(style));
    }

    let box_style = if let KanshigTuiState::OutputsFocused(_, _) = selected {
        Style::default().fg(Color::White).bold()
    } else {
        Style::default().fg(Color::Green)
    };
    let outputs_list_widget = List::new(outputs_list)
        .block(Block::new().title("Outputs").borders(Borders::ALL))
        .style(box_style);

    f.render_widget(outputs_list_widget, area);
}

fn modulo_match(selected_idx: i32, list_item_idx: i32, list_len: usize) -> bool {
    if selected_idx == 0 {
        return selected_idx == list_item_idx;
    }
    if selected_idx == -1 {
        return list_item_idx == (list_len - 1) as i32;
    }
    let item = if selected_idx < 0 {
        let offset = ((-selected_idx) + list_len as i32) % list_len as i32;
        -(selected_idx - offset)
    } else {
        selected_idx
    };
    if list_item_idx > (list_len as i32) {
        // malformed input; fail
        return false;
    }
    item % (list_len as i32) == list_item_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo_stuff() {
        assert!(modulo_match(0, 0, 3));
        assert!(modulo_match(1, 1, 3));
        assert!(modulo_match(2, 2, 3));
        assert!(modulo_match(3, 0, 3));
        assert!(modulo_match(4, 1, 3));
        assert!(modulo_match(8, 2, 3));
        assert!(!modulo_match(3, 2, 3));
        assert!(!modulo_match(2, 3, 4));
        assert!(modulo_match(-1, 2, 3));
        assert!(modulo_match(-2, 1, 3));
        assert!(modulo_match(-3, 0, 3));
        assert!(modulo_match(-4, 2, 3));
    }

    #[test]
    fn test_display_config() {
        let config = KanshiConfig {
            outputs: vec![],
            profiles: vec![],
        };

        // Just verify the function compiles
        let _ = config;
    }
}
