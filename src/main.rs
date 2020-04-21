use std::io::Write;

use anyhow::Result;
use crossterm::{
    cursor, queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{debug, info};

use app::App;

mod app;
mod commit_state;
mod context;
mod git;
mod keys;
mod log_state;
mod screen;
mod skin;
mod state;

fn init_logging() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("grit.log")?)
        .apply()?;

    Ok(())
}

fn main() -> Result<()> {
    let mut w = std::io::stderr();

    let app = clap::App::new("grit")
        .version("0.2")
        .author("Antoine Büsch <antoine.busch@gmail.com")
        .about("A text-mode UI for git")
        .arg(
            clap::Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("Enable debug logging"),
        )
        .get_matches();

    if app.is_present("debug") {
        init_logging()?;
    }

    info!("Starting grit v0.2");
    debug!("Setting up terminal");
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, cursor::Hide)?; // hiding the cursor

    let mut app = App::new();
    let result = app.run(&mut w);

    debug!("Restoring up terminal");
    terminal::disable_raw_mode()?;
    queue!(w, cursor::Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    result
}
