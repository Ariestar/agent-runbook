use crate::model::ScanMode;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandArgs {
    Help,
    Version,
    Invalid(String),
    Scan {
        mode: ScanMode,
        minimal: bool,
    },
    Category {
        categories: Vec<String>,
        lang: Option<String>,
    },
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> CommandArgs {
    let mut args = args.into_iter();
    let Some(command) = args.next() else {
        return CommandArgs::Help;
    };

    if command == "help" || command == "--help" || command == "-h" {
        return CommandArgs::Help;
    }

    if command == "version" || command == "--version" || command == "-V" {
        return CommandArgs::Version;
    }

    if command == "category" {
        return parse_category_args(args);
    }

    if command != "scan" {
        return CommandArgs::Invalid(format!("Unknown command: {command}"));
    }

    let mut mode = ScanMode::All;
    let mut minimal = false;
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
            "--minimal" => minimal = true,
            _ => return CommandArgs::Invalid(format!("Unknown option for scan: {arg}")),
        }
    }

    CommandArgs::Scan { mode, minimal }
}

fn parse_category_args(mut args: impl Iterator<Item = String>) -> CommandArgs {
    let mut categories = Vec::new();
    let mut lang = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lang" => {
                let Some(value) = args.next() else {
                    return CommandArgs::Invalid("--lang requires a value.".to_string());
                };
                lang = Some(value);
            }
            _ if arg.starts_with('-') => {
                return CommandArgs::Invalid(format!("Unknown option for category: {arg}"));
            }
            _ => categories.push(arg),
        }
    }

    CommandArgs::Category { categories, lang }
}

pub fn help_text() -> &'static str {
    "Usage:\n  runbook scan                              Scan this machine and the current project\n  runbook scan --global                     Scan only machine-level tools\n  runbook scan --local                      Scan only current-project requirements\n  runbook scan --minimal                    Print only detected tool names\n  runbook category                          List tool categories\n  runbook category <category>               List tools in a category\n  runbook category <category>... --lang rust  Filter one or more categories by language\n  runbook --version                         Print the runbook version"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_defaults_to_all_mode() {
        assert_eq!(
            parse_args(["scan"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::All,
                minimal: false
            }
        );
    }

    #[test]
    fn scan_supports_global_and_local_modes() {
        assert_eq!(
            parse_args(["scan", "--global"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::Global,
                minimal: false
            }
        );
        assert_eq!(
            parse_args(["scan", "--local"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::Local,
                minimal: false
            }
        );
    }

    #[test]
    fn scan_supports_minimal_output() {
        assert_eq!(
            parse_args(["scan", "--minimal"].into_iter().map(String::from)),
            CommandArgs::Scan {
                mode: ScanMode::All,
                minimal: true
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

    #[test]
    fn version_command_is_supported() {
        assert_eq!(
            parse_args(["--version"].into_iter().map(String::from)),
            CommandArgs::Version
        );
        assert_eq!(
            parse_args(["version"].into_iter().map(String::from)),
            CommandArgs::Version
        );
        assert_eq!(
            parse_args(["-V"].into_iter().map(String::from)),
            CommandArgs::Version
        );
    }

    #[test]
    fn category_command_lists_categories_by_default() {
        assert_eq!(
            parse_args(["category"].into_iter().map(String::from)),
            CommandArgs::Category {
                categories: Vec::new(),
                lang: None
            }
        );
    }

    #[test]
    fn category_command_accepts_categories_and_lang() {
        assert_eq!(
            parse_args(
                ["category", "lint", "formatter", "--lang", "rust"]
                    .into_iter()
                    .map(String::from)
            ),
            CommandArgs::Category {
                categories: vec!["lint".to_string(), "formatter".to_string()],
                lang: Some("rust".to_string())
            }
        );
    }

    #[test]
    fn category_command_rejects_missing_lang_value() {
        assert!(matches!(
            parse_args(
                ["category", "security", "--lang"]
                    .into_iter()
                    .map(String::from)
            ),
            CommandArgs::Invalid(_)
        ));
    }
}
