use anyhow::Result;
use crossterm::{
    cursor,
    event::KeyCode,
    queue,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::Repository;
use std::io::Write;
use termimad::{Event, EventSource};

use screen::Screen;

mod graph;
mod screen;

const UP: Event = Event::simple_key(KeyCode::Up);
const DOWN: Event = Event::simple_key(KeyCode::Down);
const J: Event = Event::simple_key(KeyCode::Char('j'));
const K: Event = Event::simple_key(KeyCode::Char('k'));
const PAGE_UP: Event = Event::simple_key(KeyCode::PageUp);
const PAGE_DOWN: Event = Event::simple_key(KeyCode::PageDown);
const HOME: Event = Event::simple_key(KeyCode::Home);
const END: Event = Event::simple_key(KeyCode::End);
const ESC: Event = Event::simple_key(KeyCode::Esc);
const ENTER: Event = Event::simple_key(KeyCode::Enter);

fn main() -> Result<()> {
    let mut w = std::io::stderr();

    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, cursor::Hide)?; // hiding the cursor

    let events = EventSource::new()?;
    let rx = events.receiver();

    let pwd = std::env::current_dir()?;
    let repo = Repository::discover(pwd)?;

    let mut screen = Screen::new(repo)?;
    loop {
        let mut quit = false;
        screen.display(&mut w)?;
        if let Ok(event) = rx.recv() {
            match event {
                UP | K => screen.commit_list.try_select_next(true),
                DOWN | J => screen.commit_list.try_select_next(false),
                PAGE_UP => screen.prev_page(),
                PAGE_DOWN => screen.next_page(),
                HOME => screen.commit_list.select_first_line(),
                END => screen.commit_list.select_last_line(),
                Event::Resize(w, h) => screen.resize(w, h),
                ENTER => {
                    if let Some(commit) = screen.commit_list.get_selection() {
                        println!("{}", commit.oid);
                    }
                }
                ESC => quit = true,
                _ => (),
            }
        } else {
            break;
        }

        events.unblock(quit);
    }

    terminal::disable_raw_mode()?;
    queue!(w, cursor::Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    Ok(())
}
