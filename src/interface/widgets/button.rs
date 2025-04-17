//  BUTTON.rs
//    by Lut99
//
//  Created:
//    17 Apr 2025, 22:55:25
//  Last edited:
//    17 Apr 2025, 23:18:24
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines a pressable "button".
//!   
//!   Buttons in a TUI are less direct. Instead, they consist of key mappings within a certain
//!   context.
//

use std::fmt::{Display, Formatter, Result as FResult};

use crossterm::event::{KeyCode, ModifierKeyCode};
use ratatui::text::Span;

use crate::state::events::Event;


/***** FORMATTERS *****/
/// Given a [`KeyCode`], displays a nice representation of the pressed button(s).
struct KeyCodeFormatter(KeyCode);
impl Display for KeyCodeFormatter {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self.0 {
            // Characters & F-keys
            KeyCode::Char(c) => write!(f, "{}", c.to_uppercase()),
            KeyCode::F(n) => write!(f, "F{n}"),

            // Special keys

            // Keys for which we don't have enough information
            KeyCode::Modifier(m) => unreachable!(),
        }
    }
}





/***** LIBRARY *****/
pub struct Button<'a> {
    /// The label of the button.
    label:   String,
    /// The trigger of the button.
    trigger: KeyCode,
    /// The output event when the button is triggered.
    output:  Box<dyn 'a + Fn() -> Event>,
}

// Constructors
impl Button<'static> {
    /// Builds a new Button.
    ///
    /// # Arguments
    /// - `label`: The text label of the button.
    /// - `trigger`: The [`KeyCode`] upon which this button should be triggered.
    /// - `output`: The output [`Event`] emitted when the button is pressed.
    ///
    /// # Returns
    /// A new Button ready to be clicked.
    #[inline]
    pub fn new(label: impl Display, trigger: KeyCode, output: Event) -> Self { Self::new_then(label, trigger, move || output) }
}
impl<'a> Button<'a> {
    /// Builds a new Button with more complicated logic to create the emitted event.
    ///
    /// # Arguments
    /// - `label`: The text label of the button.
    /// - `trigger`: The [`KeyCode`] upon which this button should be triggered.
    /// - `output`: A [mutable closure](FnMut) that creates the output [`Event`] emitted when the button is pressed.
    ///
    /// # Returns
    /// A new Button ready to be clicked.
    #[inline]
    pub fn new_then(label: impl Display, trigger: KeyCode, output: impl 'a + Fn() -> Event) -> Self {
        Self { label: label.to_string(), trigger, output: Box::new(output) }
    }
}

// Drawing
impl<'a> Button<'a> {
    /// Returns the rendered text of this Button.
    ///
    /// # Returns
    /// The emitted output [`Event`] by this Button.
    #[inline]
    pub fn draw(&self) -> Vec<Span> { vec![Span::from(format!("<{}>", self.trigger.))] }
}

// Events
impl<'a> Button<'a> {
    /// Checks whether this Button is triggered by a keypress.
    ///
    /// # Arguments
    /// - `code`: The [`KeyCode`] we potentially match on.
    ///
    /// # Returns
    /// The emitted output [`Event`] by this Button.
    #[inline]
    pub fn handle_trigger(&self, code: KeyCode) -> Option<Event> { if code == self.trigger { Some((self.output)()) } else { None } }
}
