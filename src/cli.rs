use crate::model::ScanMode;

#[derive(Debug, PartialEq, Eq)]
pub enum CommandArgs {
    Help,
    CategoryHelp,
    PreferHelp,
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
    Prefer {
        action: PreferArgs,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum PreferArgs {
    List,
    Set {
        category: String,
        lang: String,
        tool: String,
        reason: String,
    },
    Unset {
        category: String,
        lang: String,
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

    if command == "prefer" {
        return parse_prefer_args(args);
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
            "help" | "--help" | "-h" => return CommandArgs::CategoryHelp,
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

fn parse_prefer_args(mut args: impl Iterator<Item = String>) -> CommandArgs {
    let Some(action) = args.next() else {
        return CommandArgs::Prefer {
            action: PreferArgs::List,
        };
    };

    match action.as_str() {
        "help" | "--help" | "-h" => CommandArgs::PreferHelp,
        "set" => parse_prefer_set_args(args),
        "unset" => parse_prefer_unset_args(args),
        _ => CommandArgs::Invalid(format!("Unknown action for prefer: {action}")),
    }
}

fn parse_prefer_set_args(mut args: impl Iterator<Item = String>) -> CommandArgs {
    let Some(category) = args.next() else {
        return CommandArgs::Invalid("prefer set requires a category.".to_string());
    };

    let mut lang = None;
    let mut tool = None;
    let mut reason = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lang" => {
                let Some(value) = args.next() else {
                    return CommandArgs::Invalid("--lang requires a value.".to_string());
                };
                lang = Some(value);
            }
            "--tool" => {
                let Some(value) = args.next() else {
                    return CommandArgs::Invalid("--tool requires a value.".to_string());
                };
                tool = Some(value);
            }
            "--reason" => {
                let Some(value) = args.next() else {
                    return CommandArgs::Invalid("--reason requires a value.".to_string());
                };
                reason = Some(value);
            }
            _ => return CommandArgs::Invalid(format!("Unknown option for prefer set: {arg}")),
        }
    }

    let Some(lang) = lang else {
        return CommandArgs::Invalid("prefer set requires --lang <lang>.".to_string());
    };
    let Some(tool) = tool else {
        return CommandArgs::Invalid("prefer set requires --tool <tool>.".to_string());
    };
    let Some(reason) = reason else {
        return CommandArgs::Invalid("prefer set requires --reason <text>.".to_string());
    };

    CommandArgs::Prefer {
        action: PreferArgs::Set {
            category,
            lang,
            tool,
            reason,
        },
    }
}

fn parse_prefer_unset_args(mut args: impl Iterator<Item = String>) -> CommandArgs {
    let Some(category) = args.next() else {
        return CommandArgs::Invalid("prefer unset requires a category.".to_string());
    };

    let mut lang = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--lang" => {
                let Some(value) = args.next() else {
                    return CommandArgs::Invalid("--lang requires a value.".to_string());
                };
                lang = Some(value);
            }
            _ => return CommandArgs::Invalid(format!("Unknown option for prefer unset: {arg}")),
        }
    }

    let Some(lang) = lang else {
        return CommandArgs::Invalid("prefer unset requires --lang <lang>.".to_string());
    };

    CommandArgs::Prefer {
        action: PreferArgs::Unset { category, lang },
    }
}

pub fn help_text() -> &'static str {
    "Usage:\n  runbook scan                                                        Scan this machine and the current project\n  runbook scan --global                                               Scan only machine-level tools\n  runbook scan --local                                                Scan only current-project requirements\n  runbook scan --minimal                                              Print only detected tool names\n  runbook category                                                    List tool categories\n  runbook category <category>                                         List tools in a category\n  runbook category <category>... --lang rust                          Filter one or more categories by language\n  runbook category --help                                             Show category command help\n  runbook prefer                                                      List repository tool preferences\n  runbook prefer set <category> --lang <lang> --tool <tool> --reason <text>\n  runbook prefer unset <category> --lang <lang>\n  runbook prefer --help                                               Show prefer command help\n  runbook --version                                                   Print the runbook version"
}

pub fn category_help_text() -> &'static str {
    "Usage:\n  runbook category\n  runbook category <category>\n  runbook category <category>... --lang <lang>\n\nPurpose:\n  List functional tool categories or inspect candidate tools for one or more categories.\n  Tools may belong to multiple categories, so query the category that matches the task.\n\nOptions:\n  --lang <lang>    Include tools for this language plus cross-language tools.\n  -h, --help       Show this help.\n\nExamples:\n  runbook category\n  runbook category search\n  runbook category test --lang rust\n  runbook category lint formatter --lang typescript"
}

pub fn prefer_help_text() -> &'static str {
    "Usage:\n  runbook prefer\n  runbook prefer set <category> --lang <lang> --tool <tool> --reason <text>\n  runbook prefer unset <category> --lang <lang>\n\nPurpose:\n  Read or update explicit repository-local tool preferences in .runbook/preferences.yaml.\n  Preferences are written only by this command and are consumed by `runbook category`.\n\nOptions:\n  --lang <lang>      Language key for the preference.\n  --tool <tool>      Tool name, binary, or alias to prefer.\n  --reason <text>    Human reason for the preference.\n  -h, --help         Show this help.\n\nExamples:\n  runbook prefer\n  runbook prefer set test --lang rust --tool cargo --reason \"Use Cargo as the default Rust test runner.\"\n  runbook prefer unset test --lang rust"
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
    fn category_help_command_is_supported() {
        assert_eq!(
            parse_args(["category", "--help"].into_iter().map(String::from)),
            CommandArgs::CategoryHelp
        );
        assert_eq!(
            parse_args(["category", "-h"].into_iter().map(String::from)),
            CommandArgs::CategoryHelp
        );
        assert_eq!(
            parse_args(["category", "help"].into_iter().map(String::from)),
            CommandArgs::CategoryHelp
        );
    }

    #[test]
    fn prefer_help_command_is_supported() {
        assert_eq!(
            parse_args(["prefer", "--help"].into_iter().map(String::from)),
            CommandArgs::PreferHelp
        );
        assert_eq!(
            parse_args(["prefer", "-h"].into_iter().map(String::from)),
            CommandArgs::PreferHelp
        );
        assert_eq!(
            parse_args(["prefer", "help"].into_iter().map(String::from)),
            CommandArgs::PreferHelp
        );
    }

    #[test]
    fn prefer_lists_preferences_by_default() {
        assert_eq!(
            parse_args(["prefer"].into_iter().map(String::from)),
            CommandArgs::Prefer {
                action: PreferArgs::List
            }
        );
    }

    #[test]
    fn prefer_set_requires_complete_arguments() {
        assert_eq!(
            parse_args(
                [
                    "prefer",
                    "set",
                    "test",
                    "--lang",
                    "rust",
                    "--tool",
                    "cargo",
                    "--reason",
                    "Use Cargo."
                ]
                .into_iter()
                .map(String::from)
            ),
            CommandArgs::Prefer {
                action: PreferArgs::Set {
                    category: "test".to_string(),
                    lang: "rust".to_string(),
                    tool: "cargo".to_string(),
                    reason: "Use Cargo.".to_string()
                }
            }
        );
        assert!(matches!(
            parse_args(["prefer", "set", "test"].into_iter().map(String::from)),
            CommandArgs::Invalid(_)
        ));
    }

    #[test]
    fn prefer_unset_requires_lang() {
        assert_eq!(
            parse_args(
                ["prefer", "unset", "test", "--lang", "rust"]
                    .into_iter()
                    .map(String::from)
            ),
            CommandArgs::Prefer {
                action: PreferArgs::Unset {
                    category: "test".to_string(),
                    lang: "rust".to_string()
                }
            }
        );
        assert!(matches!(
            parse_args(["prefer", "unset", "test"].into_iter().map(String::from)),
            CommandArgs::Invalid(_)
        ));
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
