use crate::model::{
    Availability, CategoryCandidates, CategoryResult, CategorySummary, ToolCandidate,
};

pub fn render_category(result: &CategoryResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }

    match result {
        CategoryResult::List { categories } => render_category_list(categories),
        CategoryResult::Candidates {
            lang,
            platform,
            categories,
        } => render_category_candidates(categories, lang.as_deref(), platform.as_deref()),
    }
}

fn render_category_list(categories: &[CategorySummary]) -> String {
    if categories.is_empty() {
        return "No tool categories found.".to_string();
    }
    format!(
        "Categories: {}",
        categories
            .iter()
            .map(|category| format!(
                "{} ({} tools; {})",
                category.name,
                category.tool_count,
                category.langs.join(", ")
            ))
            .collect::<Vec<_>>()
            .join("; ")
    )
}

fn render_category_candidates(
    categories: &[CategoryCandidates],
    lang: Option<&str>,
    platform: Option<&str>,
) -> String {
    let scope = format!(
        "{}{}",
        lang.map(|value| format!(" lang={value}"))
            .unwrap_or_default(),
        platform
            .map(|value| format!(" platform={value}"))
            .unwrap_or_default()
    );
    let mut lines = vec![format!("Candidates{scope}:")];

    for category in categories {
        if category.tools.is_empty() {
            lines.push(format!("{}: none", category.name));
        } else {
            for tool in &category.tools {
                lines.push(render_tool_candidate(tool));
            }
        }
    }

    lines.join("\n")
}

fn render_tool_candidate(tool: &ToolCandidate) -> String {
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
    let mut parts = vec![
        format!("{} — {}", tool.name, compact(&tool.summary)),
        status,
        format!("risk={}", tool.risk.level),
    ];

    if !tool.use_when.is_empty() {
        parts.push(format!("use: {}", tool.use_when.join("; ")));
    }
    if !tool.avoid_when.is_empty() {
        parts.push(format!("avoid: {}", tool.avoid_when.join("; ")));
    }
    if !tool.guardrails.is_empty() {
        parts.push(format!("guardrail: {}", tool.guardrails.join("; ")));
    }
    if let Some(preference) = &tool.preference {
        parts.push(format!(
            "preferred for {}/{}: {}",
            preference.category, preference.lang, preference.reason
        ));
    }
    if !tool.risk.effects.is_empty() {
        parts.push(format!("effects={}", tool.risk.effects.join(", ")));
    }
    if let Some(score) = &tool.score {
        parts.push(format!("score={}/100 ({})", score.total, score.grade));
        if !score.summary.is_empty() {
            parts.push(score.summary.clone());
        }
    }
    parts.push(format!("docs={}", doc_url(tool)));
    parts.join("; ")
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
