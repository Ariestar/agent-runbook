use crate::model::{Availability, CategoryResult, CategorySummary, ToolCandidate};

pub fn render_category(result: &CategoryResult) -> String {
    match result {
        CategoryResult::List { categories } => render_category_list(categories),
        CategoryResult::Candidates {
            category,
            lang,
            tools,
        } => render_category_candidates(category, lang.as_deref(), tools),
    }
}

fn render_category_list(categories: &[CategorySummary]) -> String {
    let mut lines = vec![
        "Runbook Tool Categories".to_string(),
        "Use `runbook category <category> --lang <lang>` to inspect candidates.".to_string(),
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

fn render_category_candidates(
    category: &str,
    lang: Option<&str>,
    tools: &[ToolCandidate],
) -> String {
    let mut lines = vec![
        "Runbook Tool Candidates".to_string(),
        format!("Category: {category}"),
        format!("Language: {}", lang.unwrap_or("any")),
        String::new(),
        "Candidates".to_string(),
    ];

    if tools.is_empty() {
        lines.push("- None".to_string());
    } else {
        for tool in tools {
            lines.extend(render_tool_candidate(tool));
        }
    }

    lines.join("\n").trim_end().to_string()
}

fn render_tool_candidate(tool: &ToolCandidate) -> Vec<String> {
    let status = match &tool.availability {
        Availability::Found { command, version } => {
            let version = version
                .as_ref()
                .map(|value| format!("; {value}"))
                .unwrap_or_default();
            format!("available via {command}{version}")
        }
        Availability::Missing { checked } => format!("missing ({checked})"),
    };
    let aliases = if tool.aliases.is_empty() {
        String::new()
    } else {
        format!("; aliases: {}", tool.aliases.join(", "))
    };

    let mut lines = vec![
        format!("- {} [{}]: {}", tool.name, status, compact(&tool.summary)),
        format!(
            "  binary: {}; lang: {}; risk: {}{}",
            tool.binary,
            tool.langs.join(", "),
            tool.risk.level,
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
