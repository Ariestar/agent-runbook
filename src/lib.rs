mod cli;
mod commands;
mod discovery;
mod error;
mod model;
mod output;
mod registry;

pub use error::{Result, RunbookError};

use cli::{CommandArgs, parse_args};
use commands::category::{CategoryCommand, query_category};
use commands::scan::{ScanCommand, scan};
use model::{CategoryInput, ScanInput};

pub fn run() -> Result<()> {
    match parse_args(std::env::args().skip(1)) {
        CommandArgs::Help => {
            println!("{}", cli::help_text());
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
            let result = query_category(CategoryCommand {
                input: CategoryInput { categories, lang },
            });
            println!("{}", output::render_category(&result));
            Ok(())
        }
    }
}
