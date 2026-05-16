mod checks;
mod cli;
mod interpret;
mod registry;
mod render;
mod scan;

use cli::{CommandArgs, parse_args};
use scan::{ScanInput, scan};

fn main() {
    match parse_args(std::env::args().skip(1)) {
        CommandArgs::Help => {
            println!("{}", cli::help_text());
        }
        CommandArgs::Version => {
            println!("runbook {}", env!("CARGO_PKG_VERSION"));
        }
        CommandArgs::Invalid(reason) => {
            eprintln!("{reason}\n\n{}", cli::help_text());
            std::process::exit(1);
        }
        CommandArgs::Scan { mode } => {
            let cwd = match std::env::current_dir() {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("Failed to read current directory: {error}");
                    std::process::exit(1);
                }
            };

            let result = scan(ScanInput { cwd, mode });
            println!("{}", render::render_scan(&result));
        }
    }
}
