use std::env;
use std::path::PathBuf;

use crate::checks::{Fact, run_global_checks, run_local_checks};
use crate::interpret::{ScanSummary, interpret};
use crate::registry::tool_registry;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanMode {
    All,
    Global,
    Local,
}

impl ScanMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ScanMode::All => "all",
            ScanMode::Global => "global",
            ScanMode::Local => "local",
        }
    }
}

pub struct ScanInput {
    pub cwd: PathBuf,
    pub mode: ScanMode,
}

pub struct ScanResult {
    pub mode: ScanMode,
    pub cwd: PathBuf,
    pub summary: ScanSummary,
}

pub fn scan(input: ScanInput) -> ScanResult {
    let include_global = matches!(input.mode, ScanMode::All | ScanMode::Global);
    let include_local = matches!(input.mode, ScanMode::All | ScanMode::Local);
    let registry = tool_registry();
    let mut facts = Vec::new();

    if include_global {
        facts.extend(machine_facts());
    }

    for tool in &registry {
        if include_global {
            facts.extend(run_global_checks(tool));
        }

        if include_local {
            facts.extend(run_local_checks(tool, &input.cwd));
        }
    }

    let summary = interpret(input.mode, facts);
    ScanResult {
        mode: input.mode,
        cwd: input.cwd,
        summary,
    }
}

fn machine_facts() -> Vec<Fact> {
    let mut facts = vec![
        Fact::machine(
            "os",
            "Operating system",
            format!(
                "{} ({}/{})",
                env::consts::OS,
                env::consts::OS,
                env::consts::ARCH
            ),
        ),
        Fact::machine(
            "shell",
            "Shell",
            env::var("SHELL")
                .or_else(|_| env::var("ComSpec"))
                .unwrap_or_else(|_| "unknown".to_string()),
        ),
    ];

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
