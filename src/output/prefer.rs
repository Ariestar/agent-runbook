use crate::model::{PreferResult, ToolPreference};

pub fn render_prefer(result: &PreferResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }

    match result {
        PreferResult::List { path, preferences } => {
            if preferences.is_empty() {
                return format!("No repository preferences ({}).", path.display());
            }
            format!(
                "Preferences ({}): {}",
                path.display(),
                preferences
                    .iter()
                    .map(render_preference)
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        }
        PreferResult::Set { path, preference } => format!(
            "Preference saved in {}: {}/{} -> {} ({})",
            path.display(),
            preference.category,
            preference.lang,
            preference.tool,
            preference.reason
        ),
        PreferResult::Unset {
            path,
            category,
            lang,
            removed,
        } => format!(
            "Preference {category}/{lang} in {}: {}.",
            path.display(),
            if *removed { "removed" } else { "not found" }
        ),
    }
}

fn render_preference(preference: &ToolPreference) -> String {
    format!(
        "{}/{} -> {} ({})",
        preference.category, preference.lang, preference.tool, preference.reason
    )
}
