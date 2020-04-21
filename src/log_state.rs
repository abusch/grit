use std::io::Write;

use anyhow::Result;
use crossterm::{
    queue,
    terminal::{self, Clear, ClearType},
};
use git2::{Oid, Repository, RepositoryState, Sort};
use log::debug;
use termimad::{Alignment, Area, Event, ListView, ListViewCell, ListViewColumn};

use crate::{
    commit_state::CommitState,
    context::AppContext,
    git::CommitInfo,
    keys::*,
    screen::Screen,
    skin::SKIN,
    state::{AppState, CommandResult},
};

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
                Box::new(|t: &CommitInfo| ListViewCell::new(t.time.to_string(), &SKIN.commit_date)),
            ),
            ListViewColumn::new(
                "author",
                6,
                20,
                Box::new(|t: &CommitInfo| ListViewCell::new(t.author.clone(), &SKIN.commit_author)),
            )
            .with_align(Alignment::Left),
            ListViewColumn::new(
                "message",
                6,
                120,
                Box::new(|t: &CommitInfo| {
                    ListViewCell::new(t.message.clone(), &SKIN.commit_message)
                }),
            )
            .with_align(Alignment::Left),
        ];

        let (width, height) = terminal::size()?;
        let list_area = Area::new(0, 1, width, height - 2);
        let mut commit_list = ListView::new(list_area, columns, &SKIN.normal);

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
            ENTER => {
                if let Some(commit) = self.commit_list.get_selection().cloned() {
                    debug!("Opening commit {}", commit.oid);
                    let new_state = Box::new(CommitState::new(commit));
                    CommandResult::ChangeState(new_state)
                } else {
                    CommandResult::Keep
                }
            }
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
            ESC | Q => CommandResult::PopState,
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
