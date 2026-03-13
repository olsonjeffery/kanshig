use std::{cell::RefCell, fs, io::Write, rc::Rc};

use ratatui::crossterm::event;
use ratatui_textarea::TextArea;

use crate::{KanshigTuiState, model::AddOutputWindowState};

pub const MOVE_SET: &[event::KeyCode] = &[
    event::KeyCode::Up,
    event::KeyCode::Down,
    event::KeyCode::Left,
    event::KeyCode::Right,
    event::KeyCode::Char('w'),
    event::KeyCode::Char('s'),
    event::KeyCode::Char('d'),
    event::KeyCode::Char('j'),
    event::KeyCode::Char('k'),
    event::KeyCode::Tab,
];

pub fn update_input<'a>(
    selected: KanshigTuiState<'a>,
    key: event::KeyEvent,
    config_content: Option<&String>,
    config_path: &str,
    _niri_outputs: Option<&crate::model::NiriOutputs>,
    selected_unified_output: Option<&crate::model::UnifiedOutput>,
) -> KanshigTuiState<'a> {
    // Handle add output popup mode
    if let KanshigTuiState::AddOutputPopup {
        add_output_state,
        previous_outputs_index,
    } = selected.clone()
    {
        // Handle Escape to cancel
        if key.code == event::KeyCode::Esc {
            return KanshigTuiState::OutputsFocused(previous_outputs_index, previous_outputs_index);
        }

        // Handle Enter or Ctrl+Enter to submit
        if key.code == event::KeyCode::Enter {
            // Check if focus is on Add or Cancel button
            match add_output_state.focus {
                crate::model::AddOutputFocus::AddButton => {
                    // Get the values from textareas
                    let mode = add_output_state.mode.borrow().lines().join("\n");
                    let position = add_output_state.position.borrow().lines().join("\n");
                    let scale = add_output_state.scale.borrow().lines().join("\n");
                    let alias = add_output_state.alias.borrow().lines().join("\n");
                    let output_name = add_output_state.output_name.clone();

                    // Build the new output section
                    let mut new_output = format!("output \"{}\" {{\n", output_name);
                    if !mode.is_empty() {
                        new_output.push_str(&format!(" mode {}\n", mode));
                    }
                    if !position.is_empty() {
                        new_output.push_str(&format!(" position {}\n", position));
                    }
                    if !scale.is_empty() {
                        new_output.push_str(&format!(" scale {}\n", scale));
                    }
                    if !alias.is_empty() {
                        new_output.push_str(&format!(" alias {}\n", alias));
                    }
                    new_output.push_str("}\n");

                    // Append to config file
                    use std::fs::OpenOptions;
                    let result = OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(config_path)
                        .and_then(|mut f| f.write_all(new_output.as_bytes()));
                    if let Err(e) = result {
                        log::error!("Failed to append output to config: {}", e);
                    } else {
                        log::info!("Output added to config at {}", config_path);
                    }

                    return KanshigTuiState::OutputsFocused(
                        previous_outputs_index,
                        previous_outputs_index,
                    );
                }
                crate::model::AddOutputFocus::CancelButton => {
                    return KanshigTuiState::OutputsFocused(
                        previous_outputs_index,
                        previous_outputs_index,
                    );
                }
                _ => {
                    // Move focus to Add button and let next Enter press submit
                    return KanshigTuiState::AddOutputPopup {
                        add_output_state: AddOutputWindowState {
                            focus: crate::model::AddOutputFocus::AddButton,
                            ..add_output_state
                        },
                        previous_outputs_index,
                    };
                }
            }
        }

        // Handle Tab to cycle through focusable elements
        if key.code == event::KeyCode::Tab {
            let new_focus = add_output_state.focus.next();
            return KanshigTuiState::AddOutputPopup {
                add_output_state: AddOutputWindowState {
                    focus: new_focus,
                    ..add_output_state
                },
                previous_outputs_index,
            };
        }

        // Handle Shift+Tab to cycle backwards
        if key.modifiers.contains(event::KeyModifiers::SHIFT) && key.code == event::KeyCode::Tab {
            let new_focus = add_output_state.focus.previous();
            return KanshigTuiState::AddOutputPopup {
                add_output_state: AddOutputWindowState {
                    focus: new_focus,
                    ..add_output_state
                },
                previous_outputs_index,
            };
        }

        // Handle Up/Down/k/j to cycle through focusable elements
        if let event::KeyCode::Up | event::KeyCode::Char('k') = key.code {
            let new_focus = add_output_state.focus.previous();
            return KanshigTuiState::AddOutputPopup {
                add_output_state: AddOutputWindowState {
                    focus: new_focus,
                    ..add_output_state
                },
                previous_outputs_index,
            };
        }

        if let event::KeyCode::Down | event::KeyCode::Char('j') = key.code {
            let new_focus = add_output_state.focus.next();
            return KanshigTuiState::AddOutputPopup {
                add_output_state: AddOutputWindowState {
                    focus: new_focus,
                    ..add_output_state
                },
                previous_outputs_index,
            };
        }

        // Pass key to the appropriate textarea based on focus
        match add_output_state.focus {
            crate::model::AddOutputFocus::Mode => {
                add_output_state.mode.borrow_mut().input(key);
            }
            crate::model::AddOutputFocus::Position => {
                add_output_state.position.borrow_mut().input(key);
            }
            crate::model::AddOutputFocus::Scale => {
                add_output_state.scale.borrow_mut().input(key);
            }
            crate::model::AddOutputFocus::Alias => {
                add_output_state.alias.borrow_mut().input(key);
            }
            crate::model::AddOutputFocus::AddButton
            | crate::model::AddOutputFocus::CancelButton => {
                // Buttons don't receive text input
            }
        }

        return KanshigTuiState::AddOutputPopup {
            add_output_state,
            previous_outputs_index,
        };
    }

    // Handle editor mode
    if let KanshigTuiState::EditConfig {
        textarea,
        original_content,
    } = selected.clone()
    {
        // Handle save (Ctrl+S)
        if key.modifiers.contains(event::KeyModifiers::CONTROL)
            && key.code == event::KeyCode::Char('s')
        {
            let new_content = textarea.borrow().lines().join("\n");
            if let Err(e) = fs::write(config_path, &new_content) {
                log::error!("Failed to save config: {}", e);
            } else {
                log::info!("Config saved to {}", config_path);
            }
            return KanshigTuiState::OutputsFocused(0, 0);
        }

        // Handle discard (Ctrl+D)
        if key.modifiers.contains(event::KeyModifiers::CONTROL)
            && key.code == event::KeyCode::Char('d')
        {
            return KanshigTuiState::OutputsFocused(0, 0);
        }

        // Handle escape to discard
        if key.code == event::KeyCode::Esc {
            return KanshigTuiState::OutputsFocused(0, 0);
        }

        // Pass key to textarea
        textarea.borrow_mut().input(key);
        return KanshigTuiState::EditConfig {
            textarea,
            original_content,
        };
    }

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

    // Handle 'a' key to add detected-only output to config
    if key.code == event::KeyCode::Char('a') {
        if let KanshigTuiState::OutputsFocused(oi, _) = &selected {
            // Check if we have a selected unified output that is detected-only
            if let Some(output) = selected_unified_output
                && output.detected
                && !output.configured
            {
                // Get default values from niri output
                let default_mode = output.mode.clone();
                let default_position = if output.position.is_empty() {
                    "0,0".to_string()
                } else {
                    output.position.clone()
                };
                let default_scale = output.scale;

                let add_output_state = AddOutputWindowState::new(
                    &output.name,
                    &default_mode,
                    &default_position,
                    default_scale,
                );

                return KanshigTuiState::AddOutputPopup {
                    add_output_state,
                    previous_outputs_index: *oi,
                };
            }
        }
        return selected;
    }

    // Open editor with 'e' key
    if key.code == event::KeyCode::Char('e') {
        if let Some(content) = config_content {
            let textarea = TextArea::from(content.split('\n').collect::<Vec<_>>());
            //textarea.set_cursor_style(Style::default().underlined());
            return KanshigTuiState::EditConfig {
                textarea: Rc::new(RefCell::new(textarea)),
                original_content: content.clone(),
            };
        }
        return selected;
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
            KanshigTuiState::EditConfig { .. } => selected,
            KanshigTuiState::AddOutputPopup { .. } => selected,
        }
    } else {
        selected
    }
}
