use std::{cell::RefCell, fs, rc::Rc};

use ratatui::{crossterm::event, style::Style};
use ratatui_textarea::TextArea;

use crate::KanshigTuiState;

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

pub fn update_input<'a>(
    selected: KanshigTuiState<'a>,
    key: event::KeyEvent,
    config_content: Option<&String>,
    config_path: &str,
) -> KanshigTuiState<'a> {
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

    // Open editor with 'e' key
    if key.code == event::KeyCode::Char('e') {
        if let Some(content) = config_content {
            let mut textarea = TextArea::from(content.split('\n').collect::<Vec<_>>());
            textarea.set_cursor_style(Style::default().underlined());
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
        }
    } else {
        selected
    }
}
