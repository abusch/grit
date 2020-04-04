use std::io::Write;

use anyhow::Result;

use crate::state::AppState;

pub struct LogState {}

impl LogState {}

impl AppState for LogState {
    fn display(&mut self, w: &mut dyn Write) -> Result<()> {
        Ok(())
    }
}
