use crate::model::{
    Availability, CategoryCandidates, CategoryResult, CategorySummary, ToolCandidate,
};

pub fn render_category(result: &CategoryResult) -> String {
    match result {
        CategoryResult::List { categories } => render_category_list(categories),
        CategoryResult::Candidates { lang, categories } => {
            render_category_candidates(categories, lang.as_deref())
        }
    }
}

fn render_category_list(categories: &[CategorySummary]) -> String {
    let mut lines = vec![
        "Runbook Tool Categories".to_string(),
        "Use `runbook category <category>... --lang <lang>` to inspect candidates.".to_string(),
        String::new(),
        "Categories".to_string(),
    ];

    if categories.is_empty() {
        lines.push("- None".to_string());
    } else {
        lines.extend(categories.iter().map(|category| {
            format!(
                "- {}: {} tool(s); lang: {}",
                category.name,
                category.tool_count,
                category.langs.join(", ")
            )
        }));
    }

    lines.join("\n").trim_end().to_string()
}

fn render_category_candidates(categories: &[CategoryCandidates], lang: Option<&str>) -> String {
    let mut lines = vec![
        "Runbook Tool Candidates".to_string(),
        format!(
            "Categories: {}",
            categories
                .iter()
                .map(|category| category.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        format!("Language: {}", lang.unwrap_or("any")),
    ];

    for category in categories {
        lines.push(String::new());
        lines.push(format!("Category: {}", category.name));

        if category.tools.is_empty() {
            lines.push("- None".to_string());
        } else {
            for tool in &category.tools {
                lines.extend(render_tool_candidate(tool));
            }
        }
    }

    lines.join("\n").trim_end().to_string()
}

fn render_tool_candidate(tool: &ToolCandidate) -> Vec<String> {
    let mut status = match &tool.availability {
        Availability::Found { command, version } => {
            let version = version
                .as_ref()
                .map(|value| format!("; {value}"))
                .unwrap_or_default();
            format!("available via {command}{version}")
        }
        Availability::Missing { checked } => format!("missing ({checked})"),
    };
    if tool.preference.is_some() {
        status.push_str(", preferred");
    }
    let aliases = if tool.aliases.is_empty() {
        String::new()
    } else {
        format!("; aliases: {}", tool.aliases.join(", "))
    };

    let platform = if tool.platforms.is_empty() {
        String::new()
    } else {
        format!("; platform: {}", tool.platforms.join(", "))
    };

    let mut lines = vec![
        format!("- {} [{}]: {}", tool.name, status, compact(&tool.summary)),
        format!(
            "  binary: {}; lang: {}; risk: {}{}{}",
            tool.binary,
            tool.langs.join(", "),
            tool.risk.level,
            platform,
            aliases
        ),
    ];

    if !tool.use_when.is_empty() {
        lines.push(format!("  use_when: {}", tool.use_when.join("; ")));
    }
    if !tool.avoid_when.is_empty() {
        lines.push(format!("  avoid_when: {}", tool.avoid_when.join("; ")));
    }
    if !tool.guardrails.is_empty() {
        lines.push(format!("  guardrails: {}", tool.guardrails.join("; ")));
    }
    if let Some(preference) = &tool.preference {
        lines.push(format!(
            "  preferred_for: {}/{}",
            preference.category, preference.lang
        ));
        lines.push(format!("  preference_reason: {}", preference.reason));
    }
    if !tool.risk.effects.is_empty() {
        lines.push(format!("  effects: {}", tool.risk.effects.join(", ")));
    }
    lines.push(format!("  docs: {}", doc_url(tool)));

    lines
}

fn compact(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn doc_url(tool: &ToolCandidate) -> &str {
    if tool.docs.is_empty() {
        &tool.homepage
    } else {
        &tool.docs
    }
}
