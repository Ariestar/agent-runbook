use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct CommandOutput {
    pub status: bool,
    pub first_line: String,
}

pub struct CommandIndex {
    commands: HashMap<String, String>,
}

impl CommandIndex {
    pub fn new() -> Self {
        let mut commands = HashMap::new();
        let path_exts = path_exts();

        if let Some(paths) = env::var_os("PATH") {
            for directory in env::split_paths(&paths) {
                index_directory(&mut commands, &directory, &path_exts);
            }
        }

        Self { commands }
    }

    pub fn resolve(&self, command: &str) -> Option<String> {
        self.commands.get(&normalize(command)).cloned()
    }
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

fn index_directory(commands: &mut HashMap<String, String>, directory: &Path, path_exts: &[String]) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !is_command_path(&path, path_exts) {
            continue;
        }

        let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };

        let path_value = path.display().to_string();
        commands
            .entry(normalize(file_name))
            .or_insert_with(|| path_value.clone());

        if should_index_stem(&path, path_exts)
            && let Some(stem) = path.file_stem().and_then(|value| value.to_str())
        {
            commands
                .entry(normalize(stem))
                .or_insert_with(|| path_value.clone());
        }
    }
}

#[cfg(windows)]
fn path_exts() -> Vec<String> {
    env::var("PATHEXT")
        .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_string())
        .split(';')
        .filter(|value| !value.is_empty())
        .map(normalize)
        .collect()
}

#[cfg(not(windows))]
fn path_exts() -> Vec<String> {
    Vec::new()
}

#[cfg(windows)]
fn is_command_path(path: &Path, path_exts: &[String]) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|extension| path_exts.contains(&format!(".{}", normalize(extension))))
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn is_command_path(path: &Path, _path_exts: &[String]) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn should_index_stem(path: &Path, path_exts: &[String]) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|extension| path_exts.contains(&format!(".{}", normalize(extension))))
        .unwrap_or(false)
}

#[cfg(not(windows))]
fn should_index_stem(_path: &Path, _path_exts: &[String]) -> bool {
    false
}

#[cfg(windows)]
fn normalize(value: &str) -> String {
    value.to_ascii_lowercase()
}

#[cfg(not(windows))]
fn normalize(value: &str) -> String {
    value.to_string()
}
