use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Commit, Oid};

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
