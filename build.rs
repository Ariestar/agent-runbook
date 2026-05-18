use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct ToolCard {
    name: String,
    binary: String,
    #[serde(default)]
    aliases: Vec<String>,
    category: Vec<String>,
    lang: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    platform: Vec<String>,
    summary: String,
    homepage: String,
    docs: String,
    detect: Detect,
    #[serde(default)]
    use_when: Vec<String>,
    #[serde(default)]
    avoid_when: Vec<String>,
    risk: Risk,
    #[serde(default)]
    guardrails: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Detect {
    #[serde(default)]
    version_args: Vec<String>,
    #[serde(default)]
    local: LocalDetect,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct LocalDetect {
    #[serde(default)]
    files: Vec<String>,
    #[serde(default)]
    dirs: Vec<String>,
    #[serde(default)]
    package_json: PackageJsonDetect,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct PackageJsonDetect {
    #[serde(default)]
    package_manager_prefixes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Risk {
    level: String,
    #[serde(default)]
    effects: Vec<String>,
    requires_auth: bool,
    destructive: bool,
    #[serde(default)]
    confirmation_required_for: Vec<String>,
}

fn main() {
    println!("cargo:rerun-if-changed=awesome-agent-cli/data/tools");

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let tools_dir = root.join("awesome-agent-cli").join("data").join("tools");
    let mut cards = Vec::new();

    for path in yaml_files(&tools_dir) {
        println!("cargo:rerun-if-changed={}", path.display());
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        let card: ToolCard = serde_yaml::from_str(&text)
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
        validate_card(&path, &card);
        cards.push(card);
    }

    cards.sort_by(|left, right| left.name.cmp(&right.name));

    let json = serde_json::to_string_pretty(&cards).expect("failed to serialize tool index");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    fs::write(out_dir.join("tools.json"), &json).expect("failed to write generated tool index");
}

fn yaml_files(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", dir.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yaml" || extension == "yml")
        })
        .collect();
    files.sort();
    files
}

fn validate_card(path: &Path, card: &ToolCard) {
    let slug = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if slug != card.name {
        panic!(
            "{}: file name must match tool name '{}'",
            path.display(),
            card.name
        );
    }
    if card.name.trim().is_empty() {
        panic!("{}: name is required", path.display());
    }
    if card.binary.trim().is_empty() {
        panic!("{}: binary is required", path.display());
    }
    if card.category.is_empty() || card.category.iter().any(|value| value.trim().is_empty()) {
        panic!(
            "{}: category must contain at least one value",
            path.display()
        );
    }
    let mut categories = HashSet::new();
    for category in &card.category {
        if !categories.insert(category) {
            panic!("{}: duplicate category '{}'", path.display(), category);
        }
    }
    if card.lang.is_empty() || card.lang.iter().any(|value| value.trim().is_empty()) {
        panic!("{}: lang must contain at least one value", path.display());
    }
    let mut platforms = HashSet::new();
    for platform in &card.platform {
        if platform.trim().is_empty() {
            panic!("{}: platform values must not be empty", path.display());
        }
        if !platforms.insert(platform) {
            panic!("{}: duplicate platform '{}'", path.display(), platform);
        }
    }
    if card.lang.iter().any(|value| value == "all") && card.lang.len() > 1 {
        panic!(
            "{}: lang=all must not be mixed with other values",
            path.display()
        );
    }
    if !matches!(card.risk.level.as_str(), "low" | "medium" | "high") {
        panic!(
            "{}: risk.level must be low, medium, or high",
            path.display()
        );
    }
}
