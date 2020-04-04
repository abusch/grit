use crossterm::event::KeyCode;
use termimad::Event;

pub const UP: Event = Event::simple_key(KeyCode::Up);
pub const DOWN: Event = Event::simple_key(KeyCode::Down);
pub const J: Event = Event::simple_key(KeyCode::Char('j'));
pub const K: Event = Event::simple_key(KeyCode::Char('k'));
pub const PAGE_UP: Event = Event::simple_key(KeyCode::PageUp);
pub const PAGE_DOWN: Event = Event::simple_key(KeyCode::PageDown);
pub const HOME: Event = Event::simple_key(KeyCode::Home);
pub const END: Event = Event::simple_key(KeyCode::End);
pub const ESC: Event = Event::simple_key(KeyCode::Esc);
pub const Q: Event = Event::simple_key(KeyCode::Char('q'));
