mod cli;
mod commands;
mod discovery;
mod error;
mod model;
mod output;
mod preferences;
mod registry;

pub use error::{Result, RunbookError};

use clap::Parser;
use cli::{Cli, CommandArgs};
use commands::category::{CategoryCommand, query_category};
use commands::prefer::{PreferCommand, run_prefer};
use commands::scan::{ScanCommand, scan};
use model::{CategoryInput, PreferAction, PreferInput, ScanInput, ToolPreference};

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
    }
}
