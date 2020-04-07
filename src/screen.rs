use anyhow::Result;
use crossterm::{
    style::{Attribute, Color::*},
    terminal,
};
use lazy_static::lazy_static;
use termimad::{ansi, Alignment, CompoundStyle, MadSkin};

lazy_static! {
    static ref SKIN: MadSkin = make_skin();
}

pub struct Screen {
    pub dimensions: (u16, u16),
    pub skin: MadSkin,
}

impl Screen {
    pub fn new() -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            skin: make_skin(),
            dimensions: (width, height),
        })
    }
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].compound_style = CompoundStyle::with_attr(Attribute::Bold);
    skin.headers[0].align = Alignment::Left;
    skin.italic.set_fg(ansi(225));
    skin.bold = CompoundStyle::with_fg(Blue);
    skin
}
