use std::io::Write;

use anyhow::Result;
use termimad::Event;

use crate::{context::AppContext, screen::Screen};

pub enum CommandResult {
    Keep,
    ChangeState(Box<dyn AppState>),
    PopState,
}

pub trait AppState {
    fn handle_event(&mut self, event: Event) -> CommandResult;
    fn display(&mut self, w: &mut dyn Write, ctx: &AppContext, screen: &Screen) -> Result<()>;
}
