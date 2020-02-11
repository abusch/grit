use std::io::Write;

use anyhow::Result;
use chrono::{offset::FixedOffset, DateTime, TimeZone};
use crossterm::{
    queue,
    style::{Attribute, Color::*},
    terminal::{self, Clear, ClearType},
};
use git2::{Commit, Oid, Repository, RepositoryState, Sort};
use lazy_static::lazy_static;
use termimad::{
    ansi, Alignment, Area, CompoundStyle, ListView, ListViewCell, ListViewColumn, MadSkin,
};

pub struct CommitInfo {
    pub oid: Oid,
    pub time: DateTime<FixedOffset>,
    pub author: String,
    pub message: String,
}

impl CommitInfo {
    pub fn new(commit: Commit) -> Self {
        let when = commit.author().when();
        let offset = FixedOffset::east(when.offset_minutes() * 60);
        let date_time = offset.timestamp(when.seconds(), 0);
        Self {
            oid: commit.id(),
            time: date_time,
            author: commit
                .author()
                .name()
                .unwrap_or_else(|| "<invalid utf8>")
                .to_string(),
            message: commit
                .summary()
                .unwrap_or_else(|| "<invalid utf8>")
                .to_string(),
        }
    }
}

lazy_static! {
    static ref SKIN: MadSkin = make_skin();
}

pub struct Screen<'t> {
    repo: Repository,
    pub commit_list: ListView<'t, CommitInfo>,
    dimensions: (u16, u16),
    skin: &'t MadSkin,
}

impl<'t> Screen<'t> {
    pub fn new(repo: Repository) -> Result<Self> {
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

        let mut revwalk = repo.revwalk().unwrap();
        revwalk.set_sorting(Sort::TOPOLOGICAL);
        revwalk.push_head().unwrap();
        for oid in revwalk {
            let oid = oid.unwrap();
            let commit = repo.find_commit(oid).unwrap();
            commit_list.add_row(CommitInfo::new(commit));
        }
        commit_list.update_dimensions();
        commit_list.select_first_line();

        Ok(Self {
            repo,
            commit_list,
            skin: &SKIN,
            dimensions: (width, height),
        })
    }

    pub fn display<W: Write>(&mut self, w: &mut W) -> Result<()> {
        self.display_title(w)?;
        self.commit_list.write_on(w)?;
        self.display_status(w)?;

        Ok(())
    }

    pub fn prev_page(&mut self) {
        self.commit_list.unselect();
        self.commit_list.try_scroll_pages(-1);
        self.commit_list.try_select_next(false);
    }

    pub fn next_page(&mut self) {
        self.commit_list.unselect();
        self.commit_list.try_scroll_pages(1);
        self.commit_list.try_select_next(false);
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.dimensions = (width, height);
        self.commit_list.area.width = width;
        self.commit_list.area.height = height - 2;
        self.commit_list.update_dimensions();
    }

    fn display_title<W: Write>(&mut self, w: &mut W) -> Result<()> {
        let (width, _height) = self.dimensions;
        let title_area = Area::new(0, 0, width, 1);
        let state = match self.repo.state() {
            RepositoryState::Clean => "".to_string(),
            s => format!("{:?}", s),
        };

        self.skin.write_in_area_on(
            w,
            &format!(
                "# **{}**  *{}*",
                self.repo
                    .workdir()
                    .unwrap_or_else(|| self.repo.path())
                    .display(),
                state
            ),
            &title_area,
        )?;
        Ok(())
    }

    fn display_status<W: Write>(&mut self, w: &mut W) -> Result<()> {
        let (width, height) = self.dimensions;
        let oid = if let Some(commit) = self.commit_list.get_selection() {
            commit.oid
        } else {
            Oid::zero()
        };

        let status_area = Area::new(0, height - 1, width, 1);
        self.skin.write_in_area_on(
            w,
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
