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
    profiles_chunk: ratatui::prelude::Rect,
) {
    let config = match config {
        Some(t) => t.clone(),
        None => KanshiConfig::default(),
    };
    let profiles_list: Vec<ListItem> = config
        .profiles
        .iter()
        .map(|profile| {
            let output_count = profile.outputs.len();
            ListItem::new(format!("{} ({} outputs)", profile.name, output_count))
        })
        .collect();
    let profiles_list_widget = List::new(profiles_list)
        .block(Block::new().title("Profiles").borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(profiles_list_widget, profiles_chunk);
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
    for (idx, unified) in unified_outputs.iter().enumerate() {
        let is_selected = match selected {
            KanshigTuiState::OutputsFocused(oi, _, _) => *oi == idx,
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
            style.add_modifier(Modifier::BOLD)
        } else {
            style
        };

        outputs_list.push(ListItem::new(text).style(style));
    }

    let outputs_list_widget = List::new(outputs_list)
        .block(Block::new().title("Outputs").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));
    //let outputs_list_widget = outputs_list_widgetl

    f.render_widget(outputs_list_widget, area);
}

#[cfg(test)]
mod tests {
    use super::*;

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
