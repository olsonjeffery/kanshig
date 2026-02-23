//! TUI module for displaying kanshi config data

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::model::{KanshiConfig, NiriOutputs};

/// Display the kanshi config outputs and profiles
#[allow(dead_code)]
pub fn display_config(f: &mut Frame, config: &KanshiConfig) {
    let area = f.size();

    // Create two columns for outputs and profiles
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

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

    // Display profiles
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

    f.render_widget(profiles_list_widget, chunks[1]);
}

/// Display the niri outputs
#[allow(dead_code)]
pub fn display_niri_outputs(f: &mut Frame, outputs: &NiriOutputs) {
    let area = f.size();

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
