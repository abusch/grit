use crossterm::{
    cursor,
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::Color::*,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use git2::{Commit, Repository, Sort};
use lazy_static::lazy_static;
use minimad::TextTemplate;
use std::error::Error;
use std::io::Write;
use termimad::{Alignment, Area, ListView, ListViewCell, ListViewColumn, MadSkin};

lazy_static! {
    static ref SKIN: MadSkin = termimad::MadSkin::default();
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut w = std::io::stderr();

    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, cursor::Hide)?; // hiding the cursor

    let mut area = Area::full_screen();
    // area.pad(1, 1);
    // area.pad_for_max_width(120);
    let columns = vec![
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
    commit_list.try_scroll_lines(-count);
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
    commit_list.write_on(&mut w)?;
    event::read()?;

    terminal::disable_raw_mode()?;
    queue!(w, cursor::Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;

    Ok(())
}
