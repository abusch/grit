use std::io::Write;

use anyhow::Result;
use git2::DiffFormat;
use minimad::TextTemplate;
use termimad::{Area, Event, FmtText, TextView};

use crate::{
    context::AppContext,
    git::CommitInfo,
    keys::*,
    screen::Screen,
    state::{AppState, CommandResult},
};

/// Represents a line from the diff with its type
struct Line(LineType, String);

/// A type of line from a diff
enum LineType {
    FileHeader,
    HunkHeader,
    Context,
    Insertion,
    Deletion,
}

pub struct CommitState {
    commit_info: CommitInfo,
    files_changed: usize,
    insertions: usize,
    deletions: usize,
    diff_lines: Vec<Line>,
}

impl CommitState {
    pub fn new(commit: CommitInfo, ctx: &AppContext) -> Result<Self> {
        let git_commit = ctx.repo.find_commit(commit.oid)?;
        let tree = git_commit.tree()?;
        let parent_tree = git_commit.parent(0)?.tree()?;

        let diff = ctx
            .repo
            .diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
        let stats = diff.stats()?;

        let mut diff_lines = Vec::new();
        diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
            let content = String::from_utf8_lossy(line.content()).to_string();
            if line.origin() == 'F' {
                // File header: contains multiple lines so split them up
                let mut content_lines: Vec<_> = content
                    .split('\n')
                    .map(|line| Line(LineType::FileHeader, String::from(line)))
                    .collect();
                diff_lines.append(&mut content_lines);
            } else if line.origin() == 'H' {
                // Hunk header
                diff_lines.push(Line(LineType::HunkHeader, content));
            } else {
                // diff lines
                let line_str = format!("{} {}", line.origin(), content);
                let line_type = match line.origin() {
                    '+' => LineType::Insertion,
                    '-' => LineType::Deletion,
                    _ => LineType::Context,
                };
                diff_lines.push(Line(line_type, line_str));
            }
            true
        })?;

        Ok(Self {
            commit_info: commit,
            files_changed: stats.files_changed(),
            insertions: stats.insertions(),
            deletions: stats.deletions(),
            diff_lines,
        })
    }
}

impl AppState for CommitState {
    fn handle_event(&mut self, event: Event, _ctx: &AppContext) -> CommandResult {
        match event {
            Q => CommandResult::PopState,
            _ => CommandResult::Keep,
        }
    }

    fn display(&mut self, mut w: &mut dyn Write, _ctx: &AppContext, screen: &Screen) -> Result<()> {
        let (width, height) = screen.dimensions;
        screen.clear(w)?;
        let area = Area::new(0, 1, width, height - 2);
        let template = TextTemplate::from(COMMIT_INFO);
        let mut expander = template.expander();
        let oid = self.commit_info.oid.to_string();
        let files_changed = self.files_changed.to_string();
        let insertions = self.insertions.to_string();
        let deletions = self.deletions.to_string();
        expander
            .set("oid", &oid)
            .set("files_changed", &files_changed)
            .set("insertions", &insertions)
            .set("deletions", &deletions)
            .set("message", &self.commit_info.message);

        for Line(_line_type, line) in &self.diff_lines {
            expander.sub("diff").set("line", line);
        }
        let text = FmtText::from_text(&screen.skin.normal, expander.expand(), Some(width as usize));
        let tv = TextView::from(&area, &text);
        tv.write_on(&mut w)?;

        Ok(())
    }
}

const COMMIT_INFO: &str = r#"
# Commit ${oid}

${message}
---
${files_changed} files changed, ${insertions} insertions(+), ${deletions} deletions(-)

${diff
${line}
}
"#;
