use std::io::Write;

use anyhow::Result;
use minimad::TextTemplate;
use termimad::{Area, Event, FmtText, TextView};

use crate::{
    context::AppContext,
    git::CommitInfo,
    keys::*,
    screen::Screen,
    state::{AppState, CommandResult},
};

pub struct CommitState {
    commit_info: CommitInfo,
}

impl CommitState {
    pub fn new(commit: CommitInfo) -> Self {
        Self {
            commit_info: commit,
        }
    }
}

impl AppState for CommitState {
    fn handle_event(&mut self, event: Event) -> CommandResult {
        match event {
            Q => CommandResult::PopState,
            _ => CommandResult::Keep,
        }
    }

    fn display(&mut self, mut w: &mut dyn Write, ctx: &AppContext, screen: &Screen) -> Result<()> {
        let (width, height) = screen.dimensions;
        let area = Area::new(0, 1, width, height - 2);
        let template = TextTemplate::from(COMMIT_INFO);
        let mut expander = template.expander();
        let oid = self.commit_info.oid.to_string();
        expander
            .set("oid", &oid)
            .set("message", &self.commit_info.message);
        let text = FmtText::from_text(&screen.skin, expander.expand(), Some(width as usize));
        let tv = TextView::from(&area, &text);
        tv.write_on(&mut w)?;

        Ok(())
    }
}

const COMMIT_INFO: &str = r#"
# Commit ${oid}

${message}
"#;
