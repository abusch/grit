use std::io::Write;

use anyhow::Result;

pub trait AppState {
    fn display(&mut self, w: &mut dyn Write) -> Result<()>;
}
