use chrono::{naive::NaiveDateTime, offset::FixedOffset, DateTime, TimeZone, Utc};
use crossterm::{
    cursor,
    event::{self, KeyCode},
    queue,
    style::{Attribute, Color::*},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, Repository, Sort};
use lazy_static::lazy_static;
use minimad::TextTemplate;
use std::error::Error;
use std::io::Write;
use termimad::{
    ansi, Alignment, Area, CompoundStyle, Event, EventSource, ListView, ListViewCell,
    ListViewColumn, MadSkin,
};

// mod screen;
//
const UP: Event = Event::simple_key(KeyCode::Up);
const DOWN: Event = Event::simple_key(KeyCode::Down);
const PAGE_UP: Event = Event::simple_key(KeyCode::PageUp);
const PAGE_DOWN: Event = Event::simple_key(KeyCode::PageDown);
const HOME: Event = Event::simple_key(KeyCode::Home);
const END: Event = Event::simple_key(KeyCode::End);
const ESC: Event = Event::simple_key(KeyCode::Esc);

lazy_static! {
    static ref SKIN: MadSkin = make_skin();
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut w = std::io::stderr();

    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, cursor::Hide)?; // hiding the cursor

    let events = EventSource::new()?;
    let rx = events.receiver();

    let mut area = Area::full_screen();
    let columns = vec![
        ListViewColumn::new(
            "commit date",
            6,
            26,
            Box::new(|t: &Commit| {
                let when = t.author().when();
                let offset = FixedOffset::east(when.offset_minutes() * 60);
                let date_time = offset.timestamp(when.seconds(), 0);
                ListViewCell::new(date_time.to_string(), &SKIN.paragraph.compound_style)
            }),
        ),
        ListViewColumn::new(
            "author",
            6,
            20,
            Box::new(|t: &Commit| {
                ListViewCell::new(
                    t.author()
                        .name()
                        .unwrap_or_else(|| "<invalid utf8>")
                        .to_string(),
                    &SKIN.paragraph.compound_style,
                )
            }),
        ),
        ListViewColumn::new(
            "message",
            6,
            50,
            Box::new(|t: &Commit| {
                ListViewCell::new(
                    t.summary().unwrap().to_string(),
                    &SKIN.paragraph.compound_style,
                )
            }),
        )
        .with_align(Alignment::Left),
    ];
    let repo = Repository::open("/home/abusch/code/rust/rustracer").unwrap();
    let mut commit_list = ListView::new(area, columns, &SKIN);

    // println!("State: {:?}", repo.state());
    // if let Some(path) = repo.workdir() {
    //     println!("Workdir: {}", path.display());
    // } else {
    //     println!("Bare repo");
    // }

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(Sort::TOPOLOGICAL);
    revwalk.push_head().unwrap();
    let mut count = 0;
    for oid in revwalk {
        let oid = oid.unwrap();
        let commit = repo.find_commit(oid).unwrap();
        commit_list.add_row(commit);
        count += 1;
    }
    // commit_list.try_scroll_lines(-count);
    commit_list.select_first_line();
    commit_list.update_dimensions();

    commit_list.write().unwrap();

    // let statuses = repo.statuses(None).unwrap();

    // for status_entry in statuses.iter() {
    //     println!(
    //         "{:?} {}",
    //         status_entry.status(),
    //         status_entry.path().unwrap()
    //     );
    // }
    loop {
        let mut quit = false;
        commit_list.write_on(&mut w)?;
        if let Ok(event) = rx.recv() {
            match event {
                UP => commit_list.try_select_next(true),
                DOWN => commit_list.try_select_next(false),
                PAGE_UP => commit_list.try_scroll_pages(-1),
                PAGE_DOWN => commit_list.try_scroll_pages(1),
                HOME => commit_list.select_first_line(),
                END => commit_list.select_last_line(),
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

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.headers[0].compound_style = CompoundStyle::with_attr(Attribute::Bold);
    skin.headers[0].align = Alignment::Left;
    skin.italic.set_fg(ansi(225));
    skin.bold = CompoundStyle::with_fg(Blue);
    skin
}
