use lazy_static::lazy_static;

use crossterm::style::{Attribute::*, Color::AnsiValue, Color::*};
use termimad::{Alignment, CompoundStyle, MadSkin};

lazy_static! {
    pub static ref SKIN: Skin = make_skin();
}

#[derive(Clone)]
pub struct Skin {
    pub normal: MadSkin,
    pub commit_date: CompoundStyle,
    pub commit_author: CompoundStyle,
    pub commit_message: CompoundStyle,
}

impl Skin {}

fn make_skin() -> Skin {
    let mut madskin = MadSkin::default();
    madskin.headers[0].compound_style = CompoundStyle::with_attr(Bold);
    madskin.headers[0].align = Alignment::Left;
    madskin.italic.set_fg(AnsiValue(225));
    madskin.bold = CompoundStyle::with_fg(Blue);
    Skin {
        normal: madskin,
        commit_date: CompoundStyle::with_fg(Blue),
        commit_author: CompoundStyle::with_fg(Green),
        commit_message: CompoundStyle::with_fg(White),
    }
}
