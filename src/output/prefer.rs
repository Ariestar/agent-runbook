use crate::model::{PreferResult, ToolPreference};

pub fn render_prefer(result: &PreferResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }

    match result {
        PreferResult::List { path, preferences } => {
            let mut lines = vec![
                "Runbook Tool Preferences".to_string(),
                format!("File: {}", path.display()),
            ];

            if preferences.is_empty() {
                lines.push("Preferences: none".to_string());
            } else {
                lines.push("Preferences".to_string());
                for preference in preferences {
                    lines.extend(render_preference(preference));
                }
            }

            lines.join("\n").trim_end().to_string()
        }
        PreferResult::Set { path, preference } => [
            "Runbook Tool Preference Set".to_string(),
            format!("File: {}", path.display()),
            format!(
                "Preference: {}/{} -> {}",
                preference.category, preference.lang, preference.tool
            ),
            format!("Reason: {}", preference.reason),
        ]
        .join("\n"),
        PreferResult::Unset {
            path,
            category,
            lang,
            removed,
        } => [
            "Runbook Tool Preference Unset".to_string(),
            format!("File: {}", path.display()),
            format!("Preference: {category}/{lang}"),
            format!("Removed: {removed}"),
        ]
        .join("\n"),
    }
}

fn render_preference(preference: &ToolPreference) -> Vec<String> {
    vec![
        format!(
            "- {}/{} -> {}",
            preference.category, preference.lang, preference.tool
        ),
        format!("  reason: {}", preference.reason),
    ]
}
