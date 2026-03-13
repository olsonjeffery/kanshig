//! Add output window state for the popup dialog

use ratatui_textarea::TextArea;
use std::{cell::RefCell, rc::Rc};

/// Focus state for the add output window
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddOutputFocus {
    Mode = 0,
    Position = 1,
    Scale = 2,
    Alias = 3,
    AddButton = 4,
    CancelButton = 5,
}

impl AddOutputFocus {
    pub fn from_index(idx: i32) -> Self {
        match idx {
            0 => AddOutputFocus::Mode,
            1 => AddOutputFocus::Position,
            2 => AddOutputFocus::Scale,
            3 => AddOutputFocus::Alias,
            4 => AddOutputFocus::AddButton,
            5 => AddOutputFocus::CancelButton,
            _ => AddOutputFocus::Mode,
        }
    }

    pub fn to_index(&self) -> i32 {
        match self {
            AddOutputFocus::Mode => 0,
            AddOutputFocus::Position => 1,
            AddOutputFocus::Scale => 2,
            AddOutputFocus::Alias => 3,
            AddOutputFocus::AddButton => 4,
            AddOutputFocus::CancelButton => 5,
        }
    }

    pub fn next(&self) -> Self {
        Self::from_index(self.to_index() + 1)
    }

    pub fn previous(&self) -> Self {
        Self::from_index(self.to_index() - 1)
    }
}

/// State for the add output popup window
#[derive(Debug, Clone)]
pub struct AddOutputWindowState<'a> {
    pub mode: Rc<RefCell<TextArea<'a>>>,
    pub position: Rc<RefCell<TextArea<'a>>>,
    pub scale: Rc<RefCell<TextArea<'a>>>,
    pub alias: Rc<RefCell<TextArea<'a>>>,
    pub focus: AddOutputFocus,
    pub output_name: String,
}

impl<'a> AddOutputWindowState<'a> {
    pub fn new(
        output_name: &str,
        default_mode: &str,
        default_position: &str,
        default_scale: f64,
    ) -> Self {
        let mode_lines: Vec<&str> = if default_mode.is_empty() {
            vec![""]
        } else {
            vec![default_mode]
        };
        let position_lines: Vec<&str> = if default_position.is_empty() {
            vec![""]
        } else {
            vec![default_position]
        };
        let scale_str = format!("{:.1}", default_scale);
        let scale_lines: Vec<&str> = vec![&scale_str];

        Self {
            mode: Rc::new(RefCell::new(TextArea::from(mode_lines))),
            position: Rc::new(RefCell::new(TextArea::from(position_lines))),
            scale: Rc::new(RefCell::new(TextArea::from(scale_lines))),
            alias: Rc::new(RefCell::new(TextArea::from(vec![""]))),
            focus: AddOutputFocus::Mode,
            output_name: output_name.to_string(),
        }
    }

    pub fn get_mode(&self) -> String {
        self.mode.borrow().lines().join("\n")
    }

    pub fn get_position(&self) -> String {
        self.position.borrow().lines().join("\n")
    }

    pub fn get_scale(&self) -> String {
        self.scale.borrow().lines().join("\n")
    }

    pub fn get_alias(&self) -> String {
        self.alias.borrow().lines().join("\n")
    }
}
