use std::process::Command;

pub struct CommandOutput {
    pub status: bool,
    pub first_line: String,
}

pub fn resolve_command(command: &str) -> Option<String> {
    which::which(command)
        .ok()
        .map(|path| path.display().to_string())
}

pub fn run_command(command: &str, args: &[&str]) -> CommandOutput {
    let Ok(output) = Command::new(command).args(args).output() else {
        return CommandOutput {
            status: false,
            first_line: "failed to start command".to_string(),
        };
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let text = if output.status.success() {
        format!("{stdout}{stderr}")
    } else {
        format!("{stderr}{stdout}")
    };

    CommandOutput {
        status: output.status.success(),
        first_line: text
            .replace('\0', "")
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string(),
    }
}
