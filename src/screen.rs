use std::io::Write;

use anyhow::Result;
use crossterm::terminal;

use crate::skin::{Skin, SKIN};

pub struct Screen {
    pub dimensions: (u16, u16),
    pub skin: Skin,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            skin: SKIN.clone(),
            dimensions: (width, height),
        })
    }

    pub fn clear(&self, w: &mut dyn Write) -> Result<()> {
        crossterm::execute!(w, terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }
}
