use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolSpec {
    pub name: String,
    pub binary: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub category: Vec<String>,
    pub lang: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub platform: Vec<String>,
    pub summary: String,
    pub homepage: String,
    pub docs: String,
    pub detect: DetectSpec,
    #[serde(default)]
    pub use_when: Vec<String>,
    #[serde(default)]
    pub avoid_when: Vec<String>,
    pub risk: RiskSpec,
    #[serde(default)]
    pub guardrails: Vec<String>,
}

impl ToolSpec {
    pub fn supports_lang(&self, lang: &str) -> bool {
        self.lang
            .iter()
            .any(|value| value == "all" || value.eq_ignore_ascii_case(lang))
    }

    pub fn supports_platform(&self, platform: &str) -> bool {
        self.platform
            .iter()
            .any(|value| value.eq_ignore_ascii_case(platform))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DetectSpec {
    #[serde(default)]
    pub version_args: Vec<String>,
    #[serde(default)]
    pub local: LocalDetectSpec,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LocalDetectSpec {
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub dirs: Vec<String>,
    #[serde(default)]
    pub package_json: PackageJsonDetectSpec,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PackageJsonDetectSpec {
    #[serde(default)]
    pub package_manager_prefixes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RiskSpec {
    pub level: String,
    #[serde(default)]
    pub effects: Vec<String>,
    pub requires_auth: bool,
    pub destructive: bool,
    #[serde(default)]
    pub confirmation_required_for: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Fact {
    pub kind: FactKind,
    pub scope: Scope,
    pub id: Option<String>,
    pub tool_name: Option<String>,
    pub categories: Vec<String>,
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
            categories: Vec::new(),
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
            categories: Vec::new(),
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
    pub minimal: bool,
}

pub struct ScanResult {
    pub mode: ScanMode,
    pub cwd: PathBuf,
    pub minimal: bool,
    pub summary: ScanSummary,
}

pub struct ScanSummary {
    pub machine_context: Vec<Fact>,
    pub global_tools: Vec<Fact>,
    pub local_requirements: Vec<Fact>,
    pub recommendations: Vec<Message>,
    pub warnings: Vec<Message>,
}

pub struct Message {
    pub text: String,
    pub evidence: Option<String>,
}

pub struct CategoryInput {
    pub cwd: PathBuf,
    pub categories: Vec<String>,
    pub lang: Option<String>,
    pub platform: Option<String>,
    pub score: bool,
    pub profile: Option<String>,
}

pub enum CategoryResult {
    List {
        categories: Vec<CategorySummary>,
    },
    Candidates {
        lang: Option<String>,
        platform: Option<String>,
        categories: Vec<CategoryCandidates>,
    },
}

pub struct CategoryCandidates {
    pub name: String,
    pub tools: Vec<ToolCandidate>,
}

pub struct CategorySummary {
    pub name: String,
    pub tool_count: usize,
    pub langs: Vec<String>,
}

pub struct ToolCandidate {
    pub name: String,
    pub binary: String,
    pub aliases: Vec<String>,
    pub langs: Vec<String>,
    pub platforms: Vec<String>,
    pub summary: String,
    pub docs: String,
    pub homepage: String,
    pub use_when: Vec<String>,
    pub avoid_when: Vec<String>,
    pub guardrails: Vec<String>,
    pub risk: RiskSpec,
    pub availability: Availability,
    pub preference: Option<ToolPreference>,
    pub score: Option<crate::scoring::ToolScore>,
}

impl ToolCandidate {
    pub fn build(
        tool: &ToolSpec,
        preferences: &PreferenceFile,
        command_index: &crate::discovery::command::CommandIndex,
        category: &str,
        lang: Option<&str>,
    ) -> Self {
        let fact = crate::discovery::global::run_global_checks(tool, command_index, true)
            .into_iter()
            .next();
        let availability = match fact {
            Some(fact) if fact.status == Status::Found => Availability::Found {
                command: fact.command.unwrap_or_else(|| tool.binary.clone()),
                version: fact.version,
            },
            Some(fact) => Availability::Missing {
                checked: fact.value,
            },
            None => Availability::Missing {
                checked: "not checked".to_string(),
            },
        };

        Self {
            name: tool.name.clone(),
            binary: tool.binary.clone(),
            aliases: tool.aliases.clone(),
            langs: tool.lang.clone(),
            platforms: tool.platform.clone(),
            summary: tool.summary.clone(),
            docs: tool.docs.clone(),
            homepage: tool.homepage.clone(),
            use_when: tool.use_when.clone(),
            avoid_when: tool.avoid_when.clone(),
            guardrails: tool.guardrails.clone(),
            risk: tool.risk.clone(),
            availability,
            preference: crate::preferences::find_preference(
                &preferences.preferences,
                category,
                lang,
                &tool.name,
                &tool.binary,
                &tool.aliases,
            ),
            score: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Availability {
    Found {
        command: String,
        version: Option<String>,
    },
    Missing {
        checked: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToolPreference {
    pub category: String,
    pub lang: String,
    pub tool: String,
    pub reason: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PreferenceFile {
    pub schema: u32,
    pub preferences: Vec<ToolPreference>,
}

impl Default for PreferenceFile {
    fn default() -> Self {
        Self {
            schema: 1,
            preferences: Vec::new(),
        }
    }
}

pub struct PreferInput {
    pub cwd: PathBuf,
    pub action: PreferAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreferAction {
    List,
    Set(ToolPreference),
    Unset { category: String, lang: String },
}

pub enum PreferResult {
    List {
        path: PathBuf,
        preferences: Vec<ToolPreference>,
    },
    Set {
        path: PathBuf,
        preference: ToolPreference,
    },
    Unset {
        path: PathBuf,
        category: String,
        lang: String,
        removed: bool,
    },
}

#[derive(Clone, Debug)]
pub struct RecommendInput {
    pub cwd: PathBuf,
    pub task: Vec<String>,
    pub lang: Option<String>,
    pub platform: Option<String>,
    pub profile: Option<String>,
    pub rerank_url: Option<String>,
    pub api_key: Option<String>,
    pub model_cmd: Option<String>,
    pub limit: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RecommendedTool {
    pub name: String,
    pub binary: String,
    pub summary: String,
    pub category: Vec<String>,
    pub lang: Vec<String>,
    pub docs: String,
    pub homepage: String,
    pub availability: Availability,
    pub preference: Option<ToolPreference>,
    pub relevance_score: u32,
    pub match_reason: String,
    pub agent_score: crate::scoring::ToolScore,
    pub fused_score: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RecommendResult {
    pub query: String,
    pub profile: String,
    pub model_name: String,
    pub tools: Vec<RecommendedTool>,
}
