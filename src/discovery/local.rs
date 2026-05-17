use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::model::{Fact, FactKind, Scope, Status, ToolSpec};

pub fn run_local_checks(tool: &ToolSpec, cwd: &Path) -> Vec<Fact> {
    let mut facts = Vec::new();

    for file in &tool.detect.local.files {
        if cwd.join(file).exists() {
            facts.push(local_fact(tool, file.clone()));
        }
    }

    for dir in &tool.detect.local.dirs {
        if cwd.join(dir).is_dir() {
            facts.push(local_fact(tool, dir.clone()));
        }
    }

    if !tool
        .detect
        .local
        .package_json
        .package_manager_prefixes
        .is_empty()
        && let Some(package_json) = read_package_json(cwd)
        && let Some(value) = package_json.get("packageManager").and_then(Value::as_str)
    {
        for prefix in &tool.detect.local.package_json.package_manager_prefixes {
            if value.starts_with(prefix) {
                facts.push(local_fact(
                    tool,
                    format!("package.json packageManager={value}"),
                ));
            }
        }
    }

    dedupe_facts(facts)
}

fn local_fact(tool: &ToolSpec, evidence: String) -> Fact {
    Fact {
        kind: FactKind::Requirement,
        scope: Scope::Local,
        id: None,
        tool_name: Some(tool.name.clone()),
        categories: tool.category.clone(),
        command: None,
        status: Status::Found,
        label: tool.name.clone(),
        value: tool.summary.clone(),
        version: None,
        evidence: Some(evidence),
        guardrails: tool.guardrails.clone(),
        requires_global_command: true,
    }
}

fn read_package_json(cwd: &Path) -> Option<Value> {
    let content = fs::read_to_string(cwd.join("package.json")).ok()?;
    serde_json::from_str(&content).ok()
}

fn dedupe_facts(facts: Vec<Fact>) -> Vec<Fact> {
    let mut seen = Vec::new();
    let mut deduped = Vec::new();

    for fact in facts {
        let key = format!(
            "{}:{}",
            fact.label,
            fact.evidence.clone().unwrap_or_default()
        );
        if !seen.contains(&key) {
            seen.push(key);
            deduped.push(fact);
        }
    }

    deduped
}
