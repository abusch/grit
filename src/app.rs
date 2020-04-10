use std::io::Write;

use anyhow::Result;
use git2::Repository;
use log::debug;
use termimad::EventSource;

use crate::{
    context::AppContext,
    log_state::LogState,
    screen::Screen,
    state::{AppState, CommandResult},
};

pub struct App {
    states: Vec<Box<dyn AppState>>,
}

impl App {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    fn push(&mut self, state: Box<dyn AppState>) {
        self.states.push(state);
    }

    /// Pops the current state from the stack, and return true if this was the last one
    fn pop(&mut self) -> bool {
        self.states.pop();
        self.states.is_empty()
    }

    /// Return a reference to the current `AppState`
    fn state(&self) -> &dyn AppState {
        self.states.last().expect("no state found!").as_ref()
    }

    /// Return a mutable reference to the current `AppState`
    fn state_mut(&mut self) -> &mut dyn AppState {
        self.states.last_mut().expect("no state found!").as_mut()
    }

    pub fn run(&mut self, w: &mut dyn Write) -> Result<()> {
        let events = EventSource::new()?;
        let rx = events.receiver();

        debug!("Opening git repository");
        let pwd = std::env::current_dir()?;
        let repo = Repository::discover(pwd)?;

        let log_state = Box::new(LogState::new(&repo)?) as Box<dyn AppState>;
        self.push(log_state);

        let ctx = AppContext { repo };
        let screen = Screen::new()?;
        self.state_mut().display(w, &ctx, &screen)?;
        loop {
            let mut quit = false;
            if let Ok(event) = rx.recv() {
                match self.state_mut().handle_event(event) {
                    CommandResult::Keep => (),
                    CommandResult::PopState => quit = self.pop(),
                    _ => (), // ignore for now
                }
            } else {
                // When no more events, time to quit
                break;
            }

            if !quit {
                self.state_mut().display(w, &ctx, &screen)?;
            }

            events.unblock(quit);
        }

        Ok(())
    }
}
