//! `grit` is a text mode ui for git.
//!
//! (c) 2020 Antoine Busch
#![warn(missing_docs)]

/// Contains the main `App` class to run the application.
pub mod app;
mod commit_state;
mod context;
mod git;
mod keys;
mod log_state;
mod screen;
mod skin;
mod state;
