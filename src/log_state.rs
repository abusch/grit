use std::io::Write;

use anyhow::Result;
use crossterm::{
    queue,
    style::{Attribute, Color::*},
    terminal::{self, Clear, ClearType},
};
use git2::{Oid, Repository, RepositoryState, Sort};
use lazy_static::lazy_static;
use termimad::{
    ansi, Alignment, Area, CompoundStyle, Event, ListView, ListViewCell, ListViewColumn, MadSkin,
};

use crate::{
    context::AppContext,
    git::CommitInfo,
    keys::*,
    screen::Screen,
    state::{AppState, CommandResult},
};

lazy_static! {
    static ref SKIN: MadSkin = make_skin();
}

pub struct LogState<'t> {
    pub dimensions: (u16, u16),
    pub commit_list: ListView<'t, CommitInfo>,
}

impl<'t> LogState<'t> {
    pub fn new(repo: &Repository) -> Result<Self> {
        let columns = vec![
            ListViewColumn::new(
                "commit date",
                6,
                26,
                Box::new(|t: &CommitInfo| {
                    ListViewCell::new(t.time.to_string(), &SKIN.paragraph.compound_style)
                }),
            ),
            ListViewColumn::new(
                "author",
                6,
                20,
                Box::new(|t: &CommitInfo| {
                    ListViewCell::new(t.author.clone(), &SKIN.paragraph.compound_style)
                }),
            )
            .with_align(Alignment::Left),
            ListViewColumn::new(
                "message",
                6,
                120,
                Box::new(|t: &CommitInfo| {
                    ListViewCell::new(t.message.clone(), &SKIN.paragraph.compound_style)
                }),
            )
            .with_align(Alignment::Left),
        ];

        let (width, height) = terminal::size()?;
        let list_area = Area::new(0, 1, width, height - 2);
        let mut commit_list = ListView::new(list_area, columns, &SKIN);

        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(Sort::TOPOLOGICAL)?;
        revwalk.push_head()?;
        for oid in revwalk {
            let oid = oid.unwrap();
            let commit = repo.find_commit(oid).unwrap();
            commit_list.add_row(CommitInfo::new(commit));
        }
        // commit_list.select_first_line();
        commit_list.update_dimensions();
        commit_list.select_first_line();

        Ok(Self {
            dimensions: (width, height),
            commit_list,
        })
    }
}

impl<'t> AppState for LogState<'t> {
    fn handle_event(&mut self, event: Event) -> CommandResult {
        match event {
            UP | K => {
                self.commit_list.try_select_next(true);
                CommandResult::Keep
            }
            DOWN | J => {
                self.commit_list.try_select_next(false);
                CommandResult::Keep
            }
            PAGE_UP => {
                self.commit_list.unselect();
                self.commit_list.try_scroll_pages(-1);
                self.commit_list.try_select_next(false);
                CommandResult::Keep
            }
            PAGE_DOWN => {
                self.commit_list.unselect();
                self.commit_list.try_scroll_pages(1);
                self.commit_list.try_select_next(false);
                CommandResult::Keep
            }
            HOME => {
                self.commit_list.select_first_line();
                CommandResult::Keep
            }
            END => {
                self.commit_list.select_last_line();
                CommandResult::Keep
            }
            Event::Resize(w, h) => {
                self.commit_list.area.width = w;
                self.commit_list.area.height = h;
                self.commit_list.update_dimensions();
                CommandResult::Keep
            }
            ESC | Q => CommandResult::Quit,
            _ => CommandResult::Keep,
        }
    }

    fn display(&mut self, mut w: &mut dyn Write, ctx: &AppContext, screen: &Screen) -> Result<()> {
        let (width, height) = screen.dimensions;
        if (width, height) != self.dimensions {
            queue!(w, Clear(ClearType::All))?;
            self.commit_list.area.width = width;
            self.commit_list.area.height = height - 2;
            self.commit_list.update_dimensions();
        }

        let title_area = Area::new(0, 0, width, 1);
        let state = match ctx.repo.state() {
            RepositoryState::Clean => "".to_string(),
            s => format!("{:?}", s),
        };

        screen.skin.write_in_area_on(
            &mut w,
            &format!(
                "# **{}**  *{}*",
                ctx.repo
                    .workdir()
                    .unwrap_or_else(|| ctx.repo.path())
                    .display(),
                state
            ),
            &title_area,
        )?;
        self.commit_list.write_on(&mut w)?;

        let oid = if let Some(commit) = self.commit_list.get_selection() {
            commit.oid
        } else {
            Oid::zero()
        };

        let status_area = Area::new(0, height - 1, width, 1);
        screen.skin.write_in_area_on(
            &mut w,
            &format!("Press *esc* to quit, *↑,↓,PgUp,PgDn* to navigate {}", oid),
            &status_area,
        )?;

        Ok(())
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
