use crate::model::{Fact, Message, ScanMode, ScanResult};

pub fn render_scan(result: &ScanResult, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(result)
            .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e));
    }
    let mut lines = vec![format!(
        "Scan {} ({})",
        result.cwd.display(),
        result.mode.as_str()
    )];
    if result.mode != ScanMode::Local {
        push(
            &mut lines,
            "Environment",
            result
                .summary
                .machine_context
                .iter()
                .map(fact_text)
                .collect(),
        );
        push(
            &mut lines,
            "Tools",
            result.summary.global_tools.iter().map(tool_text).collect(),
        );
    }
    if result.mode != ScanMode::Global {
        push(
            &mut lines,
            "Requirements",
            result
                .summary
                .local_requirements
                .iter()
                .map(requirement_text)
                .collect(),
        );
    }
    if !result.minimal {
        push(
            &mut lines,
            "Guidance",
            result
                .summary
                .recommendations
                .iter()
                .map(message_text)
                .collect(),
        );
        push(
            &mut lines,
            "Warnings",
            result.summary.warnings.iter().map(message_text).collect(),
        );
    }
    lines.join("\n")
}

fn push(lines: &mut Vec<String>, label: &str, rows: Vec<String>) {
    if !rows.is_empty() {
        lines.push(format!("{label}: {}", rows.join("; ")));
    }
}

fn tool_text(tool: &Fact) -> String {
    format!(
        "{}={}",
        tool.label,
        tool.command.as_deref().unwrap_or("unknown")
    )
}

fn fact_text(fact: &Fact) -> String {
    format!("{}={}", fact.label, fact.value)
}

fn requirement_text(fact: &Fact) -> String {
    format!(
        "{}={}",
        fact.label,
        fact.evidence.as_deref().unwrap_or(&fact.value)
    )
}

fn message_text(message: &Message) -> String {
    message.text.clone()
}
