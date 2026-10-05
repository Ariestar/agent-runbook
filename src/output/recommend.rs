use crate::model::{Availability, RecommendResult, RecommendedTool};

pub fn render_recommend(result: &RecommendResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }

    let mut lines = vec![
        "Runbook Task Recommendations".to_string(),
        format!("Task Query: \"{}\"", result.query),
        format!(
            "Semantic Engine: {} | Profile: {} | Found {} match(es)",
            result.model_name,
            result.profile,
            result.tools.len()
        ),
    ];

    if result.tools.is_empty() {
        lines.push(String::new());
        lines.push("No relevant tools matched the query.".to_string());
        return lines.join("\n");
    }

    for (i, tool) in result.tools.iter().enumerate() {
        lines.push(String::new());
        lines.extend(render_recommended_tool(i + 1, tool));
    }

    lines.join("\n").trim_end().to_string()
}

fn render_recommended_tool(rank: usize, tool: &RecommendedTool) -> Vec<String> {
    let avail_str = match &tool.availability {
        Availability::Found { command, version } => {
            let ver = version.as_deref().map(|v| format!("; {v}")).unwrap_or_default();
            format!("available via {command}{ver}")
        }
        Availability::Missing { checked } => format!("missing ({checked})"),
    };

    let pref_badge = if tool.preference.is_some() {
        ", preferred"
    } else {
        ""
    };

    let dim_breakdown = tool
        .agent_score
        .dimensions
        .iter()
        .map(|d| format!("{}: {}/{}", d.id, d.raw_score, d.max_score))
        .collect::<Vec<_>>()
        .join(", ");

    vec![
        format!(
            "{}. {} [Fused Score: {}/100 | Grade: {}] ({}{})",
            rank, tool.name, tool.fused_score, tool.agent_score.grade, avail_str, pref_badge
        ),
        format!("   summary: {}", tool.summary),
        format!(
            "   semantic_fit: {}% ({})",
            tool.relevance_score, tool.match_reason
        ),
        format!(
            "   agent_quality: {}/100 [{}]",
            tool.agent_score.total, dim_breakdown
        ),
        format!(
            "   category: {}; lang: {}",
            tool.category.join(", "),
            tool.lang.join(", ")
        ),
        format!("   docs: {}", tool.docs),
    ]
}
