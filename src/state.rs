use std::io::Write;

use anyhow::Result;
use termimad::Event;

use crate::{context::AppContext, screen::Screen};

/// The result of applying an event (such as a key press) to the current state. This tells the main
/// loop what action to take e.g. entering a new state, or restoring the previous step.
pub enum CommandResult {
    Keep,
    ChangeState(Box<dyn AppState>),
    PopState,
}

/// Represents a state of the application.
///
/// Different states typically display differently. Examples of states are the log view, or the
/// commit view, or the status view...
pub trait AppState {
    fn handle_event(&mut self, event: Event) -> CommandResult;
    fn display(&mut self, w: &mut dyn Write, ctx: &AppContext, screen: &Screen) -> Result<()>;
}
