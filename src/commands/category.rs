use std::collections::{BTreeMap, BTreeSet};

use crate::discovery::command::CommandIndex;
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
                        command.input.platform.as_deref(),
                    ),
                    name,
                })
                .collect(),
            lang: command.input.lang,
            platform: command.input.platform,
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
    platform: Option<&str>,
) -> Vec<ToolCandidate> {
    let command_index = CommandIndex::new();
    let mut tools: Vec<ToolCandidate> = registry
        .iter()
        .filter(|tool| {
            tool.category
                .iter()
                .any(|value| value.eq_ignore_ascii_case(category))
        })
        .filter(|tool| lang.is_none_or(|value| supports_lang(tool, value)))
        .filter(|tool| platform.is_none_or(|value| supports_platform(tool, value)))
        .map(|tool| candidate(tool, preferences, &command_index, category, lang))
        .collect();

    sort_candidates(&mut tools, lang);
    tools
}

fn sort_candidates(tools: &mut [ToolCandidate], lang: Option<&str>) {
    tools.sort_by_key(|tool| {
        (
            tool.preference.is_none(),
            availability_rank(&tool.availability),
            language_rank(&tool.langs, lang),
            risk_rank(&tool.risk.level),
            tool.name.to_ascii_lowercase(),
        )
    });
}

fn availability_rank(availability: &Availability) -> u8 {
    match availability {
        Availability::Found { .. } => 0,
        Availability::Missing { .. } => 1,
    }
}

fn language_rank(langs: &[String], lang: Option<&str>) -> u8 {
    let Some(lang) = lang else {
        return 0;
    };

    if langs.iter().any(|value| value.eq_ignore_ascii_case(lang)) {
        0
    } else if langs.iter().any(|value| value == "all") {
        1
    } else {
        2
    }
}

fn risk_rank(level: &str) -> u8 {
    match level {
        "low" => 0,
        "medium" => 1,
        "high" => 2,
        "critical" => 3,
        _ => 4,
    }
}

fn supports_lang(tool: &ToolSpec, lang: &str) -> bool {
    tool.lang
        .iter()
        .any(|value| value == "all" || value.eq_ignore_ascii_case(lang))
}

fn supports_platform(tool: &ToolSpec, platform: &str) -> bool {
    tool.platform
        .iter()
        .any(|value| value.eq_ignore_ascii_case(platform))
}

fn candidate(
    tool: &ToolSpec,
    preferences: &PreferenceFile,
    command_index: &CommandIndex,
    category: &str,
    lang: Option<&str>,
) -> ToolCandidate {
    let fact = run_global_checks(tool, command_index, true)
        .into_iter()
        .next();
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
        platforms: tool.platform.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{RiskSpec, ToolPreference};

    #[test]
    fn sorting_prioritizes_preferences_then_available_tools() {
        let mut tools = vec![
            tool(
                "ast-grep",
                Availability::Missing {
                    checked: "not found".to_string(),
                },
            ),
            tool(
                "rg",
                Availability::Found {
                    command: "rg".to_string(),
                    version: None,
                },
            ),
            tool(
                "cargo-nextest",
                Availability::Missing {
                    checked: "not found".to_string(),
                },
            )
            .with_preference(),
        ];

        sort_candidates(&mut tools, Some("rust"));

        assert_eq!(
            tools
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>(),
            vec!["cargo-nextest", "rg", "ast-grep"]
        );
    }

    #[test]
    fn sorting_prefers_language_specific_and_lower_risk_tools() {
        let mut tools = vec![
            tool(
                "docker",
                Availability::Found {
                    command: "docker".to_string(),
                    version: None,
                },
            )
            .with_langs(["all"])
            .with_risk("high"),
            tool(
                "cargo",
                Availability::Found {
                    command: "cargo".to_string(),
                    version: None,
                },
            )
            .with_langs(["rust"])
            .with_risk("medium"),
            tool(
                "fd",
                Availability::Found {
                    command: "fd".to_string(),
                    version: None,
                },
            )
            .with_langs(["all"])
            .with_risk("low"),
        ];

        sort_candidates(&mut tools, Some("rust"));

        assert_eq!(
            tools
                .iter()
                .map(|tool| tool.name.as_str())
                .collect::<Vec<_>>(),
            vec!["cargo", "fd", "docker"]
        );
    }

    fn tool(name: &str, availability: Availability) -> ToolCandidate {
        ToolCandidate {
            name: name.to_string(),
            binary: name.to_string(),
            aliases: Vec::new(),
            langs: vec!["all".to_string()],
            platforms: Vec::new(),
            summary: String::new(),
            docs: String::new(),
            homepage: String::new(),
            use_when: Vec::new(),
            avoid_when: Vec::new(),
            guardrails: Vec::new(),
            risk: RiskSpec {
                level: "medium".to_string(),
                effects: Vec::new(),
                requires_auth: false,
                destructive: false,
                confirmation_required_for: Vec::new(),
            },
            availability,
            preference: None,
        }
    }

    trait CandidateBuilder {
        fn with_preference(self) -> ToolCandidate;
        fn with_langs<const N: usize>(self, langs: [&str; N]) -> ToolCandidate;
        fn with_risk(self, level: &str) -> ToolCandidate;
    }

    impl CandidateBuilder for ToolCandidate {
        fn with_preference(mut self) -> ToolCandidate {
            self.preference = Some(ToolPreference {
                category: "test".to_string(),
                lang: "rust".to_string(),
                tool: self.name.clone(),
                reason: "Repo preference".to_string(),
            });
            self
        }

        fn with_langs<const N: usize>(mut self, langs: [&str; N]) -> ToolCandidate {
            self.langs = langs.into_iter().map(String::from).collect();
            self
        }

        fn with_risk(mut self, level: &str) -> ToolCandidate {
            self.risk.level = level.to_string();
            self
        }
    }
}
