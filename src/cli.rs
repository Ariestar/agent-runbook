use clap::{Args, Parser, Subcommand};

use crate::model::ScanMode;

#[derive(Debug, Parser)]
#[command(
    name = "runbook",
    version,
    about = "Generate a local runbook for AI coding agents."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CommandArgs,
}

#[derive(Debug, Subcommand)]
pub enum CommandArgs {
    /// Scan this machine and the current project.
    Scan(ScanArgs),
    /// List functional tool categories or inspect candidates for a task.
    Category(CategoryArgs),
    /// List or update explicit repository-local tool preferences.
    Prefer(PreferCommandArgs),
}

#[derive(Debug, Args)]
pub struct ScanArgs {
    /// Scan only machine-level tools.
    #[arg(long, conflicts_with = "local")]
    pub global: bool,
    /// Scan only current-project requirements.
    #[arg(long, conflicts_with = "global")]
    pub local: bool,
    /// Print only detected tool names.
    #[arg(long)]
    pub minimal: bool,
}

impl ScanArgs {
    pub fn mode(&self) -> ScanMode {
        if self.global {
            ScanMode::Global
        } else if self.local {
            ScanMode::Local
        } else {
            ScanMode::All
        }
    }
}

#[derive(Debug, Args)]
pub struct CategoryArgs {
    /// One or more tool categories to inspect. Omit to list categories.
    pub categories: Vec<String>,
    /// Include tools for this language plus cross-language tools.
    #[arg(long)]
    pub lang: Option<String>,
}

#[derive(Debug, Args)]
pub struct PreferCommandArgs {
    #[command(subcommand)]
    pub action: Option<PreferArgs>,
}

#[derive(Debug, Subcommand)]
pub enum PreferArgs {
    /// Record a confirmed repository preference.
    Set(PreferSetArgs),
    /// Remove a stale repository preference.
    Unset(PreferUnsetArgs),
}

#[derive(Debug, Args)]
pub struct PreferSetArgs {
    /// Tool category for the preference.
    pub category: String,
    /// Language key for the preference.
    #[arg(long)]
    pub lang: String,
    /// Tool name, binary, or alias to prefer.
    #[arg(long)]
    pub tool: String,
    /// Human, repository-specific reason for the preference.
    #[arg(long)]
    pub reason: String,
}

#[derive(Debug, Args)]
pub struct PreferUnsetArgs {
    /// Tool category for the preference.
    pub category: String,
    /// Language key for the preference.
    #[arg(long)]
    pub lang: String,
}
