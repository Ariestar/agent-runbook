use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolSpec {
    pub name: String,
    pub binary: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub category: String,
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

pub fn tool_registry() -> Vec<ToolSpec> {
    serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/tools.json")))
        .expect("embedded tool index must be valid JSON")
}
