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
                PAGE_UP => {
                    screen.commit_list.unselect();
                    screen.commit_list.try_scroll_pages(-1);
                    screen.commit_list.try_select_next(false);
                }
                PAGE_DOWN => {
                    screen.commit_list.unselect();
                    screen.commit_list.try_scroll_pages(1);
                    screen.commit_list.try_select_next(false);
                }
                HOME => screen.commit_list.select_first_line(),
                END => screen.commit_list.select_last_line(),
                Event::Resize(w, h) => {
                    screen.commit_list.area.width = w;
                    screen.commit_list.area.height = h;
                    screen.commit_list.update_dimensions();
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
