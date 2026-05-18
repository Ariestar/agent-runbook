use std::env;

use crate::discovery::command::{CommandIndex, run_command};
use crate::model::{Fact, FactKind, Scope, Status, ToolSpec};

pub fn machine_facts() -> Vec<Fact> {
    let mut facts = vec![Fact::machine(
        "os",
        "Operating system",
        format!(
            "{} ({}/{})",
            env::consts::OS,
            env::consts::OS,
            env::consts::ARCH
        ),
    )];

    let proxy_names: Vec<String> = ["HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY", "NO_PROXY"]
        .iter()
        .filter(|name| env::var(name).is_ok() || env::var(name.to_lowercase()).is_ok())
        .map(|name| name.to_string())
        .collect();

    if !proxy_names.is_empty() {
        facts.push(Fact::env(
            "proxy-env",
            "Proxy environment",
            proxy_names.join(", "),
            Some("Variable values are redacted.".to_string()),
        ));
    }

    let secret_count = env::vars()
        .filter(|(name, _)| {
            let upper = name.to_ascii_uppercase();
            upper.contains("TOKEN")
                || upper.contains("SECRET")
                || upper.contains("PASSWORD")
                || upper.contains("API_KEY")
                || upper.contains("API-KEY")
                || upper.contains("ACCESS_KEY")
                || upper.contains("ACCESS-KEY")
                || upper.contains("PRIVATE_KEY")
                || upper.contains("PRIVATE-KEY")
        })
        .count();

    if secret_count > 0 {
        facts.push(Fact::env(
            "secret-like-env",
            "Secret-like environment names",
            format!("{secret_count} variable name(s) detected"),
            Some("Values are never printed.".to_string()),
        ));
    }

    facts
}

pub fn run_global_checks(
    tool: &ToolSpec,
    command_index: &CommandIndex,
    include_version: bool,
) -> Vec<Fact> {
    let command_names =
        std::iter::once(tool.binary.as_str()).chain(tool.aliases.iter().map(String::as_str));
    let mut checked = Vec::new();

    for command_name in command_names {
        checked.push(command_name.to_string());
        if let Some(resolved) = command_index.resolve(command_name) {
            let mut fact = Fact {
                kind: FactKind::Tool,
                scope: Scope::Global,
                id: None,
                tool_name: Some(tool.name.clone()),
                categories: tool.category.clone(),
                command: Some(command_name.to_string()),
                status: Status::Found,
                label: tool.name.clone(),
                value: resolved,
                version: None,
                evidence: None,
                guardrails: tool.guardrails.clone(),
                requires_global_command: false,
            };
            if include_version {
                with_version(&mut fact, command_name, &tool.detect.version_args);
            }
            return vec![fact];
        }
    }

    vec![Fact {
        kind: FactKind::Tool,
        scope: Scope::Global,
        id: None,
        tool_name: Some(tool.name.clone()),
        categories: tool.category.clone(),
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
