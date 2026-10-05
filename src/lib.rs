mod cli;
mod commands;
mod discovery;
mod error;
pub mod matching;
mod model;
mod output;
mod preferences;
mod registry;
pub mod scoring;

pub use error::{Result, RunbookError};

use clap::Parser;
use cli::{Cli, CommandArgs};
use commands::category::{CategoryCommand, query_category};
use commands::prefer::{PreferCommand, run_prefer};
use commands::recommend::{RecommendCommand, query_recommend};
use commands::scan::{ScanCommand, scan};
use model::{CategoryInput, PreferAction, PreferInput, RecommendInput, ScanInput, ToolPreference};

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CommandArgs::Scan(args) => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let result = scan(ScanCommand {
                input: ScanInput {
                    cwd,
                    mode: args.mode(),
                    minimal: args.minimal,
                },
            });
            println!("{}", output::render_scan(&result));
            Ok(())
        }
        CommandArgs::Category(args) => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let result = query_category(CategoryCommand {
                input: CategoryInput {
                    cwd,
                    categories: args.categories,
                    lang: args.lang,
                    platform: args.platform,
                    score: args.score,
                    profile: args.profile,
                },
            })?;
            println!("{}", output::render_category(&result));
            Ok(())
        }
        CommandArgs::Prefer(args) => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let action = match args.action {
                None => PreferAction::List,
                Some(cli::PreferArgs::Set(args)) => PreferAction::Set(ToolPreference {
                    category: args.category,
                    lang: args.lang,
                    tool: args.tool,
                    reason: args.reason,
                }),
                Some(cli::PreferArgs::Unset(args)) => PreferAction::Unset {
                    category: args.category,
                    lang: args.lang,
                },
            };
            let result = run_prefer(PreferCommand {
                input: PreferInput { cwd, action },
            })?;
            println!("{}", output::render_prefer(&result));
            Ok(())
        }
        CommandArgs::Recommend(args) => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let json = args.json;
            let result = query_recommend(RecommendCommand {
                input: RecommendInput {
                    cwd,
                    task: args.task,
                    lang: args.lang,
                    platform: args.platform,
                    profile: args.profile,
                    rerank_url: args.rerank_url,
                    api_key: args.api_key,
                    model_cmd: args.model_cmd,
                    limit: args.limit,
                },
            })?;
            println!("{}", output::render_recommend(&result, json));
            Ok(())
        }
    }
}
