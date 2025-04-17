//  APP.rs
//    by Lut99
//
//  Created:
//    17 Apr 2025, 22:50:18
//  Last edited:
//    17 Apr 2025, 23:09:00
//  Auto updated?
//    Yes
//
//  Description:
//!   Defines the toplevel [`App`].
//

use std::io::Stdout;

use crossterm::event;
use ratatui::prelude::CrosstermBackend;
use ratatui::{Frame, Terminal};
use thiserror::Error;


/***** ERRORS *****/
/// Defines errors originating from the [`App`] itself.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to render frame")]
    Render { source: std::io::Error },
    #[error("Failed to read interface events")]
    EventRead { source: std::io::Error },
}





/***** LIBRARY *****/
/// Manages the User Interface (UI) of the game.
#[derive(Debug)]
pub struct App {}

// Construction

// Game loop
impl App {
    /// Starts looping on the interface, producing events and receiving updates to the interface.
    ///
    /// Note that this function takes the interface by ownership. I.e., when the interface quits,
    /// the game quits.
    ///
    /// # Returns
    /// This function won't return until the user quits the app.
    pub fn run(mut self) -> Result<(), Error> {
        // We define the "real" logic in a wrapped closure such that we can handle any kind of
        // quitting w.r.t. restoring the terminal state
        let mut core = move |mut term: Terminal<CrosstermBackend<Stdout>>| -> Result<(), Error> {
            loop {
                // Render the new UI state (immediate mode and all that)
                log::trace!("Rendering terminal UI");
                if let Err(err) = term.draw(|frame| self.draw(frame)) {
                    ratatui::restore();
                    return Err(Error::Render { source: err });
                }

                // Handle any events
                match event::read() {
                    Ok(event) => {
                        if self.handle_event(event) {
                            break;
                        }
                    },
                    Err(err) => return Err(Error::EventRead { source: err }),
                }
            }
            Ok(())
        };

        // Run the core function, quitting the terminal at every turn
        let res = core(ratatui::init());
        ratatui::restore();
        res
    }
}

// Events
impl App {
    /// Handles events inputted by the user.
    ///
    /// This is simply a translation layer over the internal [`State::handle_event()`]. This allows
    /// the state to operate at a much higher level.
    ///
    /// # Arguments
    /// - `event`: The [`crossterm`] [`event::Event`] that we will process.
    ///
    /// # Returns
    /// Whether the app should quit (true) or not (false).
    pub fn handle_event(&mut self, event: event::Event) -> bool { true }
}

// Rendering
impl App {
    /// Renders the interface to the given terminal.
    ///
    /// # Arguments
    /// - `frame`: A [`Frame`] to render to.
    pub fn draw(&self, frame: &mut Frame) {}
}
