mod cli;
mod commands;
mod discovery;
mod error;
mod model;
mod output;
mod preferences;
mod registry;

pub use error::{Result, RunbookError};

use cli::{CommandArgs, parse_args};
use commands::category::{CategoryCommand, query_category};
use commands::prefer::{PreferCommand, run_prefer};
use commands::scan::{ScanCommand, scan};
use model::{CategoryInput, PreferAction, PreferInput, ScanInput, ToolPreference};

pub fn run() -> Result<()> {
    match parse_args(std::env::args().skip(1)) {
        CommandArgs::Help => {
            println!("{}", cli::help_text());
            Ok(())
        }
        CommandArgs::CategoryHelp => {
            println!("{}", cli::category_help_text());
            Ok(())
        }
        CommandArgs::PreferHelp => {
            println!("{}", cli::prefer_help_text());
            Ok(())
        }
        CommandArgs::Version => {
            println!("runbook {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        CommandArgs::Invalid(reason) => Err(RunbookError::usage(reason, cli::help_text())),
        CommandArgs::Scan { mode, minimal } => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let result = scan(ScanCommand {
                input: ScanInput { cwd, mode, minimal },
            });
            println!("{}", output::render_scan(&result));
            Ok(())
        }
        CommandArgs::Category { categories, lang } => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let result = query_category(CategoryCommand {
                input: CategoryInput {
                    cwd,
                    categories,
                    lang,
                },
            })?;
            println!("{}", output::render_category(&result));
            Ok(())
        }
        CommandArgs::Prefer { action } => {
            let cwd = std::env::current_dir().map_err(RunbookError::current_dir)?;
            let action = match action {
                cli::PreferArgs::List => PreferAction::List,
                cli::PreferArgs::Set {
                    category,
                    lang,
                    tool,
                    reason,
                } => PreferAction::Set(ToolPreference {
                    category,
                    lang,
                    tool,
                    reason,
                }),
                cli::PreferArgs::Unset { category, lang } => PreferAction::Unset { category, lang },
            };
            let result = run_prefer(PreferCommand {
                input: PreferInput { cwd, action },
            })?;
            println!("{}", output::render_prefer(&result));
            Ok(())
        }
    }
}
