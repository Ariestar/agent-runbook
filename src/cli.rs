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
    /// Recommend tools for a natural language task with model-driven semantic scoring.
    Recommend(RecommendArgs),
    /// Run as a Model Context Protocol (MCP) server over stdio for AI agents.
    Mcp,
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
    /// Output scan result as an explicit machine-readable JSON contract.
    #[arg(long)]
    pub json: bool,
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
    /// Include only tools for this platform.
    #[arg(long)]
    pub platform: Option<String>,
    /// Calculate and display Agent-Ready Scores for candidates.
    #[arg(long)]
    pub score: bool,
    /// Scoring profile: balanced, safety, automation, or minimal.
    #[arg(long)]
    pub profile: Option<String>,
    /// Output categories and candidates as an explicit machine-readable JSON contract.
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Args)]
pub struct PreferCommandArgs {
    #[command(subcommand)]
    pub action: Option<PreferArgs>,
    /// Output tool preferences as an explicit machine-readable JSON contract.
    #[arg(long)]
    pub json: bool,
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

#[derive(Debug, Args)]
pub struct RecommendArgs {
    /// Natural language task requirement or query (e.g. "parse pdf tables to markdown").
    pub task: Vec<String>,
    /// Limit search to tools supporting this language.
    #[arg(long)]
    pub lang: Option<String>,
    /// Limit search to tools supporting this platform.
    #[arg(long)]
    pub platform: Option<String>,
    /// Scoring weight profile: balanced, safety, automation, or minimal.
    #[arg(long)]
    pub profile: Option<String>,
    /// Remote Reranker or Embedding endpoint (e.g. Jina Rerank or local TEI/Ollama).
    #[arg(long)]
    pub rerank_url: Option<String>,
    /// API key for remote model endpoint.
    #[arg(long)]
    pub api_key: Option<String>,
    /// External CLI script or model command for local scoring.
    #[arg(long)]
    pub model_cmd: Option<String>,
    /// Maximum number of recommended tools to return.
    #[arg(long, default_value = "5")]
    pub limit: usize,
    /// Output recommendations as an explicit machine-readable JSON contract.
    #[arg(long)]
    pub json: bool,
}
