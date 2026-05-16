use std::fmt;

pub type Result<T> = std::result::Result<T, RunbookError>;

#[derive(Debug)]
pub enum RunbookError {
    CurrentDir(std::io::Error),
    Usage { reason: String, help: &'static str },
}

impl RunbookError {
    pub fn current_dir(error: std::io::Error) -> Self {
        Self::CurrentDir(error)
    }

    pub fn usage(reason: String, help: &'static str) -> Self {
        Self::Usage { reason, help }
    }
}

impl fmt::Display for RunbookError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentDir(error) => {
                write!(formatter, "Failed to read current directory: {error}")
            }
            Self::Usage { reason, help } => write!(formatter, "{reason}\n\n{help}"),
        }
    }
}

impl std::error::Error for RunbookError {}
