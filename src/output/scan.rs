use std::collections::BTreeSet;

use crate::model::{Fact, Message, ScanMode, ScanResult};

pub fn render_scan(result: &ScanResult) -> String {
    let mut lines = Vec::new();

    lines.push("Agent Runbook Scan".to_string());
    lines.push(format!("Mode: {}", result.mode.as_str()));
    lines.push(format!("Project: {}", result.cwd.display()));
    lines.push(String::new());

    if result.minimal {
        return render_minimal_scan(result, lines);
    }

    if result.mode != ScanMode::Local {
        section(
            &mut lines,
            "Global Tools",
            render_global_tools(&result.summary.global_tools),
        );
    }

    if result.mode != ScanMode::Global {
        section(
            &mut lines,
            "Local Requirements",
            render_local_requirements(&result.summary.local_requirements),
        );
    }

    section(
        &mut lines,
        "Recommended Operating Guardrails",
        render_messages(&result.summary.recommendations),
    );
    section(
        &mut lines,
        "Warnings",
        render_messages(&result.summary.warnings),
    );

    lines.join("\n").trim_end().to_string()
}

fn render_minimal_scan(result: &ScanResult, mut lines: Vec<String>) -> String {
    if result.mode != ScanMode::Local {
        section(
            &mut lines,
            "Global Tools",
            render_tool_names(&result.summary.global_tools),
        );
    }

    if result.mode != ScanMode::Global {
        section(
            &mut lines,
            "Local Requirements",
            render_tool_names(&result.summary.local_requirements),
        );
    }

    lines.join("\n").trim_end().to_string()
}

fn section(lines: &mut Vec<String>, title: &str, rows: Vec<String>) {
    lines.push(title.to_string());
    if rows.is_empty() {
        lines.push("- None".to_string());
    } else {
        lines.extend(rows);
    }
    lines.push(String::new());
}

fn render_global_tools(tools: &[Fact]) -> Vec<String> {
    tools
        .iter()
        .map(|tool| {
            let version = tool
                .version
                .as_ref()
                .map(|value| format!(" ({value})"))
                .unwrap_or_default();
            format!(
                "- {}: {}{}",
                tool.label,
                tool.command.as_deref().unwrap_or("unknown"),
                version
            )
        })
        .collect()
}

fn render_local_requirements(requirements: &[Fact]) -> Vec<String> {
    requirements
        .iter()
        .map(|requirement| {
            format!(
                "- {}: {}",
                requirement.label,
                requirement
                    .evidence
                    .as_deref()
                    .unwrap_or(&requirement.value)
            )
        })
        .collect()
}

fn render_messages(messages: &[Message]) -> Vec<String> {
    messages
        .iter()
        .map(|message| {
            let evidence = message
                .evidence
                .as_ref()
                .map(|value| format!(" [{value}]"))
                .unwrap_or_default();
            format!("- {}{}", message.text, evidence)
        })
        .collect()
}

fn render_tool_names(tools: &[Fact]) -> Vec<String> {
    tools
        .iter()
        .map(|tool| tool.label.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|name| format!("- {name}"))
        .collect()
}
