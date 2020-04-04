use std::io::Write;

use anyhow::Result;
use git2::Repository;
use termimad::{Event, EventSource};

use crate::{keys::*, screen::Screen, state::AppState};

pub struct App {
    states: Vec<Box<dyn AppState>>,
}

impl App {
    pub fn new() -> Self {
        Self { states: Vec::new() }
    }

    pub fn run(&mut self, w: &mut dyn Write) -> Result<()> {
        let events = EventSource::new()?;
        let rx = events.receiver();

        let pwd = std::env::current_dir()?;
        let repo = Repository::discover(pwd)?;

        let mut screen = Screen::new(repo)?;
        loop {
            let mut quit = false;
            screen.display(w)?;
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
                    ESC | Q => quit = true,
                    _ => (),
                }
            } else {
                break;
            }

            events.unblock(quit);
        }

        Ok(())
    }
}
