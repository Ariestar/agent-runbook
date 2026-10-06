use crate::model::{Availability, RecommendResult, RecommendedTool};

pub fn render_recommend(result: &RecommendResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }

    if result.tools.is_empty() {
        return format!("No relevant tools matched: \"{}\".", result.query);
    }
    let mut lines = vec![format!(
        "Recommendations for \"{}\" ({}; profile={}):",
        result.query, result.model_name, result.profile
    )];
    lines.extend(
        result
            .tools
            .iter()
            .enumerate()
            .map(|(i, tool)| render_recommended_tool(i + 1, tool)),
    );
    lines.join("\n")
}

fn render_recommended_tool(rank: usize, tool: &RecommendedTool) -> String {
    let avail_str = match &tool.availability {
        Availability::Found { command, version } => {
            let ver = version
                .as_deref()
                .map(|v| format!("; {v}"))
                .unwrap_or_default();
            format!("available via {command}{ver}")
        }
        Availability::Missing { checked } => format!("missing ({checked})"),
    };

    let pref_badge = if tool.preference.is_some() {
        ", preferred"
    } else {
        ""
    };

    format!(
        "{}. {} — {}; {}; match={}%; score={}/100 ({}); {}{}; category={}; lang={}; docs={}",
        rank,
        tool.name,
        tool.summary,
        avail_str,
        tool.relevance_score,
        tool.fused_score,
        tool.agent_score.grade,
        tool.match_reason,
        pref_badge,
        tool.category.join(", "),
        tool.lang.join(", "),
        tool.docs
    )
}
