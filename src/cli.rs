use crate::scan::ScanMode;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandArgs {
    Help,
    Invalid(String),
    Scan { mode: ScanMode },
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> CommandArgs {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return CommandArgs::Help;
    };

    if command == "help" || command == "--help" || command == "-h" {
        return CommandArgs::Help;
    }

    if command != "scan" {
        return CommandArgs::Invalid(format!("Unknown command: {command}"));
    }

    let mut mode = ScanMode::All;
    for arg in args {
        match arg.as_str() {
            "--global" => {
                if mode == ScanMode::Local {
                    return CommandArgs::Invalid(
                        "Use either --global or --local, not both.".to_string(),
                    );
                }
                mode = ScanMode::Global;
            }
            "--local" => {
                if mode == ScanMode::Global {
                    return CommandArgs::Invalid(
                        "Use either --global or --local, not both.".to_string(),
                    );
                }
                mode = ScanMode::Local;
            }
            _ => return CommandArgs::Invalid(format!("Unknown option for scan: {arg}")),
        }
    }

    CommandArgs::Scan { mode }
}

pub fn help_text() -> &'static str {
    "Usage:\n  runbook scan          Scan this machine and the current project\n  runbook scan --global Scan only machine-level tools\n  runbook scan --local  Scan only current-project requirements"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_defaults_to_all_mode() {
        assert_eq!(
            parse_args(["scan"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::All
            }
        );
    }

    #[test]
    fn scan_supports_global_and_local_modes() {
        assert_eq!(
            parse_args(["scan", "--global"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::Global
            }
        );
        assert_eq!(
            parse_args(["scan", "--local"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::Local
            }
        );
    }

    #[test]
    fn scan_rejects_conflicting_modes() {
        assert!(matches!(
            parse_args(
                ["scan", "--global", "--local"]
                    .into_iter()
                    .map(String::from)
            ),
            CommandArgs::Invalid(_)
        ));
    }
}
