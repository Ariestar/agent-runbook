use crate::discovery::command::CommandIndex;
use crate::discovery::global::{machine_facts, run_global_checks};
use crate::discovery::local::run_local_checks;
use crate::model::{
    Fact, FactKind, Message, ScanInput, ScanMode, ScanResult, ScanSummary, Scope, Status,
};
use crate::registry::tool_registry;

pub struct ScanCommand {
    pub input: ScanInput,
}

pub fn scan(command: ScanCommand) -> ScanResult {
    let include_global = matches!(command.input.mode, ScanMode::All | ScanMode::Global);
    let include_local = matches!(command.input.mode, ScanMode::All | ScanMode::Local);
    let registry = tool_registry();
    let mut facts = Vec::new();

    if include_global {
        facts.extend(machine_facts());
    }

    let command_index = include_global.then(CommandIndex::new);

    for tool in &registry {
        if let Some(command_index) = &command_index {
            facts.extend(run_global_checks(
                tool,
                command_index,
                !command.input.minimal,
            ));
        }

        if include_local {
            facts.extend(run_local_checks(tool, &command.input.cwd));
        }
    }

    let summary = interpret(command.input.mode, facts);
    ScanResult {
        mode: command.input.mode,
        cwd: command.input.cwd,
        minimal: command.input.minimal,
        summary,
    }
}

fn interpret(mode: ScanMode, facts: Vec<Fact>) -> ScanSummary {
    let global_tools: Vec<Fact> = facts
        .iter()
        .filter(|fact| {
            fact.scope == Scope::Global
                && fact.kind == FactKind::Tool
                && fact.status == Status::Found
        })
        .cloned()
        .collect();
    let local_requirements: Vec<Fact> = facts
        .iter()
        .filter(|fact| fact.scope == Scope::Local && fact.kind == FactKind::Requirement)
        .cloned()
        .collect();

    let recommendations = build_recommendations(&facts, &local_requirements);
    let warnings = build_warnings(mode, &global_tools, &local_requirements);

    ScanSummary {
        global_tools,
        local_requirements,
        recommendations,
        warnings,
    }
}

fn build_recommendations(facts: &[Fact], local_requirements: &[Fact]) -> Vec<Message> {
    let mut recommendations = Vec::new();

    if let Some(package_manager) = choose_package_manager(local_requirements) {
        add_message(
            &mut recommendations,
            format!(
                "Use {} for package commands.",
                package_manager.tool_name.as_deref().unwrap_or("unknown")
            ),
            package_manager.evidence.clone(),
        );
    }

    for requirement in local_requirements {
        for guardrail in &requirement.guardrails {
            add_message(
                &mut recommendations,
                guardrail.clone(),
                requirement.evidence.clone(),
            );
        }
    }

    if facts
        .iter()
        .any(|fact| fact.id.as_deref() == Some("secret-like-env"))
    {
        add_message(
            &mut recommendations,
            "Do not print raw environment variables; redact secret-like values.".to_string(),
            Some("secret-like env names detected".to_string()),
        );
    }

    recommendations
}

fn build_warnings(
    mode: ScanMode,
    global_tools: &[Fact],
    local_requirements: &[Fact],
) -> Vec<Message> {
    let mut warnings = Vec::new();

    if mode != ScanMode::Local {
        for requirement in local_requirements {
            let tool_name = requirement.tool_name.as_deref().unwrap_or_default();
            let available = global_tools
                .iter()
                .any(|tool| tool.tool_name.as_deref() == Some(tool_name));
            if requirement.requires_global_command && !available {
                add_message(
                    &mut warnings,
                    format!(
                        "Project expects {}, but no matching command was found globally.",
                        requirement.label
                    ),
                    requirement.evidence.clone(),
                );
            }
        }
    }

    let package_managers: Vec<&str> = local_requirements
        .iter()
        .filter(|requirement| {
            requirement
                .categories
                .iter()
                .any(|category| category == "package-manager")
        })
        .filter_map(|requirement| requirement.tool_name.as_deref())
        .collect();

    if package_managers.contains(&"pnpm") {
        add_message(
            &mut warnings,
            "This project indicates pnpm; avoid npm install unless the user explicitly asks."
                .to_string(),
            Some("pnpm project evidence".to_string()),
        );
    }

    warnings
}

fn choose_package_manager(local_requirements: &[Fact]) -> Option<&Fact> {
    ["pnpm", "yarn", "bun", "npm"].iter().find_map(|tool_name| {
        local_requirements
            .iter()
            .find(|requirement| requirement.tool_name.as_deref() == Some(*tool_name))
    })
}

fn add_message(messages: &mut Vec<Message>, text: String, evidence: Option<String>) {
    if !messages.iter().any(|message| message.text == text) {
        messages.push(Message { text, evidence });
    }
}
