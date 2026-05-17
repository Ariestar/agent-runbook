use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{Result, RunbookError},
    model::{PreferenceFile, ToolPreference},
};

const PREFERENCE_DIR: &str = ".runbook";
const PREFERENCE_FILE: &str = "preferences.yaml";

pub fn preference_path(cwd: &Path) -> PathBuf {
    cwd.join(PREFERENCE_DIR).join(PREFERENCE_FILE)
}

pub fn load_preferences(cwd: &Path) -> Result<PreferenceFile> {
    let path = preference_path(cwd);
    if !path.exists() {
        return Ok(PreferenceFile::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|source| RunbookError::io("read", path.clone(), source))?;
    serde_yaml::from_str(&content)
        .map_err(|source| RunbookError::preference_parse(path.clone(), source))
}

pub fn save_preferences(cwd: &Path, preferences: &PreferenceFile) -> Result<PathBuf> {
    let path = preference_path(cwd);
    let Some(parent) = path.parent() else {
        return Ok(path);
    };

    fs::create_dir_all(parent)
        .map_err(|source| RunbookError::io("create directory", parent.to_path_buf(), source))?;
    let content = serde_yaml::to_string(preferences)
        .map_err(|source| RunbookError::preference_write(path.clone(), source))?;
    fs::write(&path, content).map_err(|source| RunbookError::io("write", path.clone(), source))?;
    Ok(path)
}

pub fn find_preference(
    preferences: &[ToolPreference],
    category: &str,
    lang: Option<&str>,
    name: &str,
    binary: &str,
    aliases: &[String],
) -> Option<ToolPreference> {
    preferences
        .iter()
        .find(|preference| {
            preference.category.eq_ignore_ascii_case(category)
                && lang.is_none_or(|value| preference.lang.eq_ignore_ascii_case(value))
                && matches_tool(&preference.tool, name, binary, aliases)
        })
        .cloned()
}

fn matches_tool(expected: &str, name: &str, binary: &str, aliases: &[String]) -> bool {
    expected.eq_ignore_ascii_case(name)
        || expected.eq_ignore_ascii_case(binary)
        || aliases
            .iter()
            .any(|alias| expected.eq_ignore_ascii_case(alias))
}
