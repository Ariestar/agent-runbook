use crate::{
    error::Result,
    model::{PreferAction, PreferInput, PreferResult, ToolPreference},
    preferences::{load_preferences, preference_path, save_preferences},
};

pub struct PreferCommand {
    pub input: PreferInput,
}

pub fn run_prefer(command: PreferCommand) -> Result<PreferResult> {
    match command.input.action {
        PreferAction::List => {
            let preferences = load_preferences(&command.input.cwd)?;
            Ok(PreferResult::List {
                path: preference_path(&command.input.cwd),
                preferences: preferences.preferences,
            })
        }
        PreferAction::Set(preference) => set_preference(command.input.cwd, preference),
        PreferAction::Unset { category, lang } => {
            unset_preference(command.input.cwd, category, lang)
        }
    }
}

fn set_preference(cwd: std::path::PathBuf, preference: ToolPreference) -> Result<PreferResult> {
    let mut preferences = load_preferences(&cwd)?;
    if let Some(existing) = preferences.preferences.iter_mut().find(|existing| {
        existing.category.eq_ignore_ascii_case(&preference.category)
            && existing.lang.eq_ignore_ascii_case(&preference.lang)
    }) {
        *existing = preference.clone();
    } else {
        preferences.preferences.push(preference.clone());
    }

    let path = save_preferences(&cwd, &preferences)?;
    Ok(PreferResult::Set { path, preference })
}

fn unset_preference(
    cwd: std::path::PathBuf,
    category: String,
    lang: String,
) -> Result<PreferResult> {
    let mut preferences = load_preferences(&cwd)?;
    let before = preferences.preferences.len();
    preferences.preferences.retain(|preference| {
        !(preference.category.eq_ignore_ascii_case(&category)
            && preference.lang.eq_ignore_ascii_case(&lang))
    });
    let removed = preferences.preferences.len() != before;
    let path = save_preferences(&cwd, &preferences)?;

    Ok(PreferResult::Unset {
        path,
        category,
        lang,
        removed,
    })
}
