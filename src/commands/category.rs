use std::collections::{BTreeMap, BTreeSet};

use crate::discovery::global::run_global_checks;
use crate::model::{
    Availability, CategoryInput, CategoryResult, CategorySummary, Status, ToolCandidate, ToolSpec,
};
use crate::registry::tool_registry;

pub struct CategoryCommand {
    pub input: CategoryInput,
}

pub fn query_category(command: CategoryCommand) -> CategoryResult {
    let registry = tool_registry();
    match command.input.category {
        Some(category) => CategoryResult::Candidates {
            tools: candidates(&registry, &category, command.input.lang.as_deref()),
            category,
            lang: command.input.lang,
        },
        None => CategoryResult::List {
            categories: summaries(&registry),
        },
    }
}

fn summaries(registry: &[ToolSpec]) -> Vec<CategorySummary> {
    let mut categories: BTreeMap<String, (usize, BTreeSet<String>)> = BTreeMap::new();

    for tool in registry {
        let entry = categories
            .entry(tool.category.clone())
            .or_insert_with(|| (0, BTreeSet::new()));
        entry.0 += 1;
        entry.1.extend(tool.lang.iter().cloned());
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

fn candidates(registry: &[ToolSpec], category: &str, lang: Option<&str>) -> Vec<ToolCandidate> {
    registry
        .iter()
        .filter(|tool| tool.category.eq_ignore_ascii_case(category))
        .filter(|tool| lang.is_none_or(|value| supports_lang(tool, value)))
        .map(candidate)
        .collect()
}

fn supports_lang(tool: &ToolSpec, lang: &str) -> bool {
    tool.lang
        .iter()
        .any(|value| value == "all" || value.eq_ignore_ascii_case(lang))
}

fn candidate(tool: &ToolSpec) -> ToolCandidate {
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
    }
}
