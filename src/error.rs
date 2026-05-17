use std::{fmt, path::PathBuf};

pub type Result<T> = std::result::Result<T, RunbookError>;

#[derive(Debug)]
pub enum RunbookError {
    CurrentDir(std::io::Error),
    Io {
        action: &'static str,
        path: PathBuf,
        source: std::io::Error,
    },
    PreferenceParse {
        path: PathBuf,
        source: serde_yaml::Error,
    },
    PreferenceWrite {
        path: PathBuf,
        source: serde_yaml::Error,
    },
    Usage {
        reason: String,
        help: &'static str,
    },
}

impl RunbookError {
    pub fn current_dir(error: std::io::Error) -> Self {
        Self::CurrentDir(error)
    }

    pub fn usage(reason: String, help: &'static str) -> Self {
        Self::Usage { reason, help }
    }

    pub fn io(action: &'static str, path: PathBuf, source: std::io::Error) -> Self {
        Self::Io {
            action,
            path,
            source,
        }
    }

    pub fn preference_parse(path: PathBuf, source: serde_yaml::Error) -> Self {
        Self::PreferenceParse { path, source }
    }

    pub fn preference_write(path: PathBuf, source: serde_yaml::Error) -> Self {
        Self::PreferenceWrite { path, source }
    }
}

impl fmt::Display for RunbookError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDir(error) => {
                write!(formatter, "Failed to read current directory: {error}")
            }
            Self::Io {
                action,
                path,
                source,
            } => write!(formatter, "Failed to {action} {}: {source}", path.display()),
            Self::PreferenceParse { path, source } => write!(
                formatter,
                "Failed to parse preference file {}: {source}",
                path.display()
            ),
            Self::PreferenceWrite { path, source } => write!(
                formatter,
                "Failed to serialize preference file {}: {source}",
                path.display()
            ),
            Self::Usage { reason, help } => write!(formatter, "{reason}\n\n{help}"),
        }
    }
}

impl std::error::Error for RunbookError {}
