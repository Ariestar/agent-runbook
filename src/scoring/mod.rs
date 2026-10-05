use crate::model::{Availability, ToolCandidate};

/// Context passed to scorers when evaluating candidates.
#[derive(Clone, Debug, Default)]
pub struct ScoreContext<'a> {
    pub lang: Option<&'a str>,
    pub category: Option<&'a str>,
    pub platform: Option<&'a str>,
}

/// A single evaluated dimension of an agent tool.
#[derive(Clone, Debug, PartialEq)]
pub struct ScoredDimension {
    pub id: String,
    pub name: String,
    pub raw_score: u32,
    pub max_score: u32,
    pub weight: f32,
    pub reasoning: String,
}

impl ScoredDimension {
    pub fn weighted_score(&self) -> f32 {
        if self.max_score == 0 {
            0.0
        } else {
            (self.raw_score as f32 / self.max_score as f32) * self.weight * 100.0
        }
    }
}

/// Overall composite score for an agent tool candidate.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolScore {
    pub total: u32,
    pub grade: String,
    pub summary: String,
    pub dimensions: Vec<ScoredDimension>,
}

/// Pluggable trait for scoring tools across specific agent capability dimensions.
pub trait ToolScorer: Send + Sync {
    /// Unique identifier for this scorer plugin.
    fn id(&self) -> &'static str;
    /// Human-readable label for this dimension.
    fn name(&self) -> &'static str;
    /// Evaluate a candidate and produce a scored dimension.
    fn score(&self, candidate: &ToolCandidate, context: &ScoreContext) -> ScoredDimension;
}

/// Weight profile determining the emphasis of different scoring dimensions.
#[derive(Clone, Debug, PartialEq)]
pub enum ScoreProfile {
    Balanced,
    SafetyFirst,
    Automation,
    Minimal,
    Custom {
        readability: f32,
        safety: f32,
        efficiency: f32,
        maturity: f32,
    },
}

impl ScoreProfile {
    pub fn from_name(name: Option<&str>) -> Self {
        match name.map(|s| s.to_ascii_lowercase()).as_deref() {
            Some("safety") | Some("safety-first") | Some("safety_first") => ScoreProfile::SafetyFirst,
            Some("automation") | Some("auto") => ScoreProfile::Automation,
            Some("minimal") | Some("lean") => ScoreProfile::Minimal,
            _ => ScoreProfile::Balanced,
        }
    }

    pub fn weight_for(&self, scorer_id: &str) -> f32 {
        match self {
            ScoreProfile::Balanced => 0.25,
            ScoreProfile::SafetyFirst => match scorer_id {
                "safety" => 0.45,
                "readability" => 0.20,
                "efficiency" => 0.20,
                "maturity" => 0.15,
                _ => 0.10,
            },
            ScoreProfile::Automation => match scorer_id {
                "readability" => 0.40,
                "efficiency" => 0.30,
                "safety" => 0.20,
                "maturity" => 0.10,
                _ => 0.10,
            },
            ScoreProfile::Minimal => match scorer_id {
                "efficiency" => 0.40,
                "safety" => 0.30,
                "readability" => 0.20,
                "maturity" => 0.10,
                _ => 0.10,
            },
            ScoreProfile::Custom {
                readability,
                safety,
                efficiency,
                maturity,
            } => match scorer_id {
                "readability" => *readability,
                "safety" => *safety,
                "efficiency" => *efficiency,
                "maturity" => *maturity,
                _ => 0.10,
            },
        }
    }
}

/// Pluggable evaluator for machine readability, structured formats, and headless automation.
pub struct ReadabilityScorer;

impl ToolScorer for ReadabilityScorer {
    fn id(&self) -> &'static str {
        "readability"
    }

    fn name(&self) -> &'static str {
        "Machine Readability"
    }

    fn score(&self, candidate: &ToolCandidate, _context: &ScoreContext) -> ScoredDimension {
        let mut points: u32 = 12; // Base baseline
        let mut reasons = Vec::new();

        let text_corpus = format!(
            "{} {} {}",
            candidate.summary.to_lowercase(),
            candidate.use_when.join(" ").to_lowercase(),
            candidate.avoid_when.join(" ").to_lowercase()
        );

        // Check structured output signals
        if text_corpus.contains("json") || text_corpus.contains("structured") || text_corpus.contains("ast") {
            points += 5;
            reasons.push("supports structured data/json");
        }

        // Check non-interactive or batch friendly indicators
        if text_corpus.contains("batch") || text_corpus.contains("scriptable") || text_corpus.contains("headless") {
            points += 4;
            reasons.push("scriptable/headless friendly");
        } else {
            points += 2;
        }

        // Check clear binary declaration
        if !candidate.binary.is_empty() {
            points += 4;
        }

        let raw_score = points.min(25);
        let reasoning = if reasons.is_empty() {
            "standard command line interface".to_string()
        } else {
            reasons.join(", ")
        };

        ScoredDimension {
            id: self.id().to_string(),
            name: self.name().to_string(),
            raw_score,
            max_score: 25,
            weight: 0.25,
            reasoning,
        }
    }
}

/// Pluggable evaluator for safety, reversibility, and blast-radius containment.
pub struct SafetyScorer;

impl ToolScorer for SafetyScorer {
    fn id(&self) -> &'static str {
        "safety"
    }

    fn name(&self) -> &'static str {
        "Safety & Guardrails"
    }

    fn score(&self, candidate: &ToolCandidate, _context: &ScoreContext) -> ScoredDimension {
        let mut points: u32 = match candidate.risk.level.as_str() {
            "low" => 22,
            "medium" => 16,
            "high" => 10,
            _ => 12,
        };

        let mut reasons: Vec<String> = Vec::new();
        match candidate.risk.level.as_str() {
            "low" => reasons.push("low inherent risk".to_string()),
            "medium" => reasons.push("moderate risk scope".to_string()),
            "high" => reasons.push("high operational privileges required".to_string()),
            _ => {}
        }

        if candidate.risk.destructive {
            points = points.saturating_sub(6);
            reasons.push("destructive mutations possible".to_string());
        } else {
            points += 2;
        }

        if !candidate.guardrails.is_empty() {
            points += 3;
            reasons.push(format!("{} guardrails defined", candidate.guardrails.len()));
        }

        let raw_score = points.min(25);

        ScoredDimension {
            id: self.id().to_string(),
            name: self.name().to_string(),
            raw_score,
            max_score: 25,
            weight: 0.25,
            reasoning: reasons.join(", "),
        }
    }
}

/// Pluggable evaluator for token efficiency, side effect brevity, and output footprint.
pub struct EfficiencyScorer;

impl ToolScorer for EfficiencyScorer {
    fn id(&self) -> &'static str {
        "efficiency"
    }

    fn name(&self) -> &'static str {
        "Context Efficiency"
    }

    fn score(&self, candidate: &ToolCandidate, _context: &ScoreContext) -> ScoredDimension {
        let mut points: u32 = 14;
        let mut reasons: Vec<String> = Vec::new();

        // Penalize heavy mutations that bloat execution traces
        let effects_count = candidate.risk.effects.len();
        if effects_count <= 2 {
            points += 6;
            reasons.push("focused side-effects".to_string());
        } else if effects_count <= 4 {
            points += 3;
            reasons.push("standard effect footprint".to_string());
        } else {
            points = points.saturating_sub(2);
            reasons.push("broad cross-domain side effects".to_string());
        }

        // Reward concise summaries with high information density
        let summary_words = candidate.summary.split_whitespace().count();
        if (5..=25).contains(&summary_words) {
            points += 5;
            reasons.push("concise documentation".to_string());
        } else {
            points += 2;
        }

        let raw_score = points.min(25);

        ScoredDimension {
            id: self.id().to_string(),
            name: self.name().to_string(),
            raw_score,
            max_score: 25,
            weight: 0.25,
            reasoning: reasons.join(", "),
        }
    }
}

/// Pluggable evaluator for ecosystem maturity, availability, and project documentation.
pub struct MaturityScorer;

impl ToolScorer for MaturityScorer {
    fn id(&self) -> &'static str {
        "maturity"
    }

    fn name(&self) -> &'static str {
        "Ecosystem Maturity"
    }

    fn score(&self, candidate: &ToolCandidate, _context: &ScoreContext) -> ScoredDimension {
        let mut points: u32 = 8;
        let mut reasons: Vec<String> = Vec::new();

        match &candidate.availability {
            Availability::Found { command, .. } => {
                points += 10;
                reasons.push(format!("available locally via {command}"));
            }
            Availability::Missing { .. } => {
                points += 3;
                reasons.push("not currently installed".to_string());
            }
        }

        if !candidate.docs.is_empty() && candidate.docs.starts_with("https://") {
            points += 4;
            reasons.push("complete https documentation".to_string());
        }

        if !candidate.homepage.is_empty() {
            points += 3;
        }

        let raw_score = points.min(25);

        ScoredDimension {
            id: self.id().to_string(),
            name: self.name().to_string(),
            raw_score,
            max_score: 25,
            weight: 0.25,
            reasoning: reasons.join(", "),
        }
    }
}

/// Extensible scoring pipeline that manages scorers and calculates composite ratings.
pub struct ScoringPipeline {
    scorers: Vec<Box<dyn ToolScorer>>,
    profile: ScoreProfile,
}

impl Default for ScoringPipeline {
    fn default() -> Self {
        Self {
            scorers: vec![
                Box::new(ReadabilityScorer),
                Box::new(SafetyScorer),
                Box::new(EfficiencyScorer),
                Box::new(MaturityScorer),
            ],
            profile: ScoreProfile::Balanced,
        }
    }
}

impl ScoringPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_profile(mut self, profile: ScoreProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn from_profile_name(name: Option<&str>) -> Self {
        Self::default().with_profile(ScoreProfile::from_name(name))
    }

    /// Register a custom scorer plugin into the pipeline.
    pub fn register<S: ToolScorer + 'static>(&mut self, scorer: S) {
        self.scorers.push(Box::new(scorer));
    }

    /// Clear all built-in scorers to construct a purely customized pipeline.
    pub fn clear_scorers(&mut self) {
        self.scorers.clear();
    }

    /// Evaluate a candidate against all registered scorers using the active profile.
    pub fn score_candidate(&self, candidate: &ToolCandidate, context: &ScoreContext) -> ToolScore {
        let mut dimensions = Vec::with_capacity(self.scorers.len());
        let mut total_weighted = 0.0f32;

        for scorer in &self.scorers {
            let mut dim = scorer.score(candidate, context);
            dim.weight = self.profile.weight_for(scorer.id());
            total_weighted += dim.weighted_score();
            dimensions.push(dim);
        }

        let total = (total_weighted.round() as u32).min(100);

        let grade = match total {
            90..=100 => "A+",
            80..=89 => "A",
            70..=79 => "B",
            60..=69 => "C",
            _ => "D",
        }
        .to_string();

        let top_reasons: Vec<&str> = dimensions
            .iter()
            .map(|d| d.reasoning.as_str())
            .filter(|r| !r.is_empty())
            .take(2)
            .collect();

        let summary = top_reasons.join("; ");

        ToolScore {
            total,
            grade,
            summary,
            dimensions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::RiskSpec;

    fn mock_candidate(name: &str, risk_level: &str, found: bool) -> ToolCandidate {
        ToolCandidate {
            name: name.to_string(),
            binary: name.to_string(),
            aliases: vec![],
            langs: vec!["all".to_string()],
            platforms: vec![],
            summary: "Command line developer tool with structured JSON output.".to_string(),
            docs: "https://example.com/docs".to_string(),
            homepage: "https://example.com".to_string(),
            use_when: vec!["Automated tasks".to_string()],
            avoid_when: vec![],
            guardrails: vec!["Review diff before applying".to_string()],
            risk: RiskSpec {
                level: risk_level.to_string(),
                effects: vec!["read_files".to_string()],
                requires_auth: false,
                destructive: false,
                confirmation_required_for: vec![],
            },
            availability: if found {
                Availability::Found {
                    command: name.to_string(),
                    version: Some("1.0.0".to_string()),
                }
            } else {
                Availability::Missing {
                    checked: "not found".to_string(),
                }
            },
            preference: None,
            score: None,
        }
    }

    #[test]
    fn pipeline_scores_and_grades_candidate() {
        let pipeline = ScoringPipeline::default();
        let candidate = mock_candidate("test-tool", "low", true);
        let context = ScoreContext::default();
        let score = pipeline.score_candidate(&candidate, &context);

        assert!(score.total >= 80, "Expected high score, got {}", score.total);
        assert!(score.grade == "A" || score.grade == "A+");
        assert_eq!(score.dimensions.len(), 4);
    }

    #[test]
    fn safety_profile_weights_safety_higher() {
        let balanced = ScoringPipeline::default().with_profile(ScoreProfile::Balanced);
        let safety = ScoringPipeline::default().with_profile(ScoreProfile::SafetyFirst);

        let high_risk = mock_candidate("risky-tool", "high", false);
        let context = ScoreContext::default();

        let balanced_score = balanced.score_candidate(&high_risk, &context);
        let safety_score = safety.score_candidate(&high_risk, &context);

        assert!(
            safety_score.total < balanced_score.total,
            "Safety profile should penalize high risk more: safety={} vs balanced={}",
            safety_score.total,
            balanced_score.total
        );
    }

    struct MockCustomScorer;
    impl ToolScorer for MockCustomScorer {
        fn id(&self) -> &'static str {
            "custom_latency"
        }
        fn name(&self) -> &'static str {
            "Execution Latency"
        }
        fn score(&self, _candidate: &ToolCandidate, _context: &ScoreContext) -> ScoredDimension {
            ScoredDimension {
                id: "custom_latency".to_string(),
                name: "Execution Latency".to_string(),
                raw_score: 25,
                max_score: 25,
                weight: 0.1,
                reasoning: "sub-10ms response".to_string(),
            }
        }
    }

    #[test]
    fn pipeline_supports_pluggable_custom_scorers() {
        let mut pipeline = ScoringPipeline::default();
        pipeline.register(MockCustomScorer);

        let candidate = mock_candidate("fast-tool", "low", true);
        let context = ScoreContext::default();
        let score = pipeline.score_candidate(&candidate, &context);

        assert_eq!(score.dimensions.len(), 5);
        assert!(score.dimensions.iter().any(|d| d.id == "custom_latency"));
    }
}
