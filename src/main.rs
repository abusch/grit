use anyhow::Result;
use crossterm::{
    cursor, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Write;

use app::App;

mod app;
mod context;
mod git;
mod keys;
mod log_state;
mod screen;
mod state;

fn main() -> Result<()> {
    let mut w = std::io::stderr();

    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, cursor::Hide)?; // hiding the cursor

    let mut app = App::new();
    let result = app.run(&mut w);

    terminal::disable_raw_mode()?;
    queue!(w, cursor::Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    result
}
