use std::collections::{BTreeMap, BTreeSet};

use crate::discovery::global::run_global_checks;
use crate::error::Result;
use crate::model::{
    Availability, CategoryCandidates, CategoryInput, CategoryResult, CategorySummary,
    PreferenceFile, Status, ToolCandidate, ToolSpec,
};
use crate::preferences::{find_preference, load_preferences};
use crate::registry::tool_registry;

pub struct CategoryCommand {
    pub input: CategoryInput,
}

pub fn query_category(command: CategoryCommand) -> Result<CategoryResult> {
    let registry = tool_registry();
    if command.input.categories.is_empty() {
        Ok(CategoryResult::List {
            categories: summaries(&registry),
        })
    } else {
        let preferences = load_preferences(&command.input.cwd)?;
        Ok(CategoryResult::Candidates {
            categories: command
                .input
                .categories
                .into_iter()
                .map(|name| CategoryCandidates {
                    tools: candidates(
                        &registry,
                        &preferences,
                        &name,
                        command.input.lang.as_deref(),
                    ),
                    name,
                })
                .collect(),
            lang: command.input.lang,
        })
    }
}

fn summaries(registry: &[ToolSpec]) -> Vec<CategorySummary> {
    let mut categories: BTreeMap<String, (usize, BTreeSet<String>)> = BTreeMap::new();

    for tool in registry {
        for category in &tool.category {
            let entry = categories
                .entry(category.clone())
                .or_insert_with(|| (0, BTreeSet::new()));
            entry.0 += 1;
            entry.1.extend(tool.lang.iter().cloned());
        }
    }

    categories
        .into_iter()
        .map(|(name, (tool_count, langs))| CategorySummary {
            name,
            tool_count,
            langs: langs.into_iter().collect(),
        })
        .collect()
}

fn candidates(
    registry: &[ToolSpec],
    preferences: &PreferenceFile,
    category: &str,
    lang: Option<&str>,
) -> Vec<ToolCandidate> {
    let mut tools: Vec<ToolCandidate> = registry
        .iter()
        .filter(|tool| {
            tool.category
                .iter()
                .any(|value| value.eq_ignore_ascii_case(category))
        })
        .filter(|tool| lang.is_none_or(|value| supports_lang(tool, value)))
        .map(|tool| candidate(tool, preferences, category, lang))
        .collect();

    tools.sort_by_key(|tool| tool.preference.is_none());
    tools
}

fn supports_lang(tool: &ToolSpec, lang: &str) -> bool {
    tool.lang
        .iter()
        .any(|value| value == "all" || value.eq_ignore_ascii_case(lang))
}

fn candidate(
    tool: &ToolSpec,
    preferences: &PreferenceFile,
    category: &str,
    lang: Option<&str>,
) -> ToolCandidate {
    let fact = run_global_checks(tool).into_iter().next();
    let availability = match fact {
        Some(fact) if fact.status == Status::Found => Availability::Found {
            command: fact.command.unwrap_or_else(|| tool.binary.clone()),
            version: fact.version,
        },
        Some(fact) => Availability::Missing {
            checked: fact.value,
        },
        None => Availability::Missing {
            checked: "not checked".to_string(),
        },
    };

    ToolCandidate {
        name: tool.name.clone(),
        binary: tool.binary.clone(),
        aliases: tool.aliases.clone(),
        langs: tool.lang.clone(),
        summary: tool.summary.clone(),
        docs: tool.docs.clone(),
        homepage: tool.homepage.clone(),
        use_when: tool.use_when.clone(),
        avoid_when: tool.avoid_when.clone(),
        guardrails: tool.guardrails.clone(),
        risk: tool.risk.clone(),
        availability,
        preference: find_preference(
            &preferences.preferences,
            category,
            lang,
            &tool.name,
            &tool.binary,
            &tool.aliases,
        ),
    }
}
