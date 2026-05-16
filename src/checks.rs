use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

use crate::registry::ToolSpec;

#[derive(Debug, Clone)]
pub struct Fact {
    pub kind: FactKind,
    pub scope: Scope,
    pub id: Option<String>,
    pub tool_name: Option<String>,
    pub category: Option<String>,
    pub command: Option<String>,
    pub status: Status,
    pub label: String,
    pub value: String,
    pub version: Option<String>,
    pub evidence: Option<String>,
    pub guardrails: Vec<String>,
    pub requires_global_command: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactKind {
    Tool,
    Requirement,
    Machine,
    Env,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Global,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Found,
    Missing,
}

impl Fact {
    pub fn machine(id: &str, label: &str, value: String) -> Self {
        Self {
            kind: FactKind::Machine,
            scope: Scope::Global,
            id: Some(id.to_string()),
            tool_name: None,
            category: None,
            command: None,
            status: Status::Found,
            label: label.to_string(),
            value,
            version: None,
            evidence: None,
            guardrails: Vec::new(),
            requires_global_command: false,
        }
    }

    pub fn env(id: &str, label: &str, value: String, note: Option<String>) -> Self {
        Self {
            kind: FactKind::Env,
            scope: Scope::Global,
            id: Some(id.to_string()),
            tool_name: None,
            category: None,
            command: None,
            status: Status::Found,
            label: label.to_string(),
            value,
            version: None,
            evidence: note,
            guardrails: Vec::new(),
            requires_global_command: false,
        }
    }
}

pub fn run_global_checks(tool: &ToolSpec) -> Vec<Fact> {
    let command_names =
        std::iter::once(tool.binary.as_str()).chain(tool.aliases.iter().map(String::as_str));
    let mut checked = Vec::new();

    for command_name in command_names {
        checked.push(command_name.to_string());
        if let Some(resolved) = resolve_command(command_name) {
            let mut fact = Fact {
                kind: FactKind::Tool,
                scope: Scope::Global,
                id: None,
                tool_name: Some(tool.name.clone()),
                category: Some(tool.category.clone()),
                command: Some(command_name.to_string()),
                status: Status::Found,
                label: tool.name.clone(),
                value: resolved,
                version: None,
                evidence: None,
                guardrails: tool.guardrails.clone(),
                requires_global_command: false,
            };
            with_version(&mut fact, command_name, &tool.detect.version_args);
            return vec![fact];
        }
    }

    vec![Fact {
        kind: FactKind::Tool,
        scope: Scope::Global,
        id: None,
        tool_name: Some(tool.name.clone()),
        category: Some(tool.category.clone()),
        command: Some(tool.binary.clone()),
        status: Status::Missing,
        label: tool.name.clone(),
        value: format!("not found; checked {}", checked.join(", ")),
        version: None,
        evidence: None,
        guardrails: tool.guardrails.clone(),
        requires_global_command: false,
    }]
}

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
        category: Some(tool.category.clone()),
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

fn with_version(fact: &mut Fact, command_name: &str, args: &[String]) {
    if args.is_empty() {
        return;
    }

    let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = run_command(command_name, &arg_refs);
    if output.status && !output.first_line.is_empty() {
        fact.version = Some(output.first_line);
    }
}

fn resolve_command(command: &str) -> Option<String> {
    let output = if cfg!(windows) {
        run_command("where", &[command])
    } else {
        run_command("sh", &["-c", &format!("command -v {command}")])
    };

    output
        .status
        .then_some(output.first_line)
        .filter(|line| !line.is_empty())
}

struct CommandOutput {
    status: bool,
    first_line: String,
}

fn run_command(command: &str, args: &[&str]) -> CommandOutput {
    let output = Command::new(command).args(args).output().or_else(|error| {
        if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", &format_command_line(command, args)])
                .output()
        } else {
            Err(error)
        }
    });

    let Ok(output) = output else {
        return CommandOutput {
            status: false,
            first_line: "failed to start command".to_string(),
        };
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let text = if output.status.success() {
        format!("{stdout}{stderr}")
    } else {
        format!("{stderr}{stdout}")
    };

    CommandOutput {
        status: output.status.success(),
        first_line: text
            .replace('\0', "")
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .to_string(),
    }
}

fn format_command_line(command: &str, args: &[&str]) -> String {
    std::iter::once(command)
        .chain(args.iter().copied())
        .map(quote_command_part)
        .collect::<Vec<_>>()
        .join(" ")
}

fn quote_command_part(value: &str) -> String {
    if !value
        .chars()
        .any(|ch| ch.is_whitespace() || "\"&|<>^()".contains(ch))
    {
        return value.to_string();
    }

    format!("\"{}\"", value.replace('"', "\\\""))
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
