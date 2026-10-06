use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::io::Write;
use std::process::{Command, Stdio};

/// Pluggable trait for model-driven semantic relevance scoring.
pub trait SemanticScorer: Send + Sync {
    /// Human-friendly display label.
    fn name(&self) -> &'static str;
    /// Score an array of candidate documents against a task query.
    /// Returns a vector of relevance scores in the range [0.0, 1.0], matching document order.
    fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32>;
}

/// HTTP Rerank model client (supports Jina Reranker, TEI, Cohere, or OpenAI-compatible endpoints).
pub struct HttpRerankScorer {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub model: String,
}

/// Generic System One decision-model client for Jev, Laya, and compatible local servers.
pub struct SystemOneScorer {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub max_options: usize,
}

impl SystemOneScorer {
    pub fn new(
        endpoint: String,
        api_key: Option<String>,
        model: Option<String>,
        max_options: usize,
    ) -> Self {
        Self {
            endpoint,
            api_key,
            model,
            max_options: max_options.max(2),
        }
    }
}

impl SemanticScorer for SystemOneScorer {
    fn name(&self) -> &'static str {
        "System One Decision Model"
    }

    fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32> {
        if documents.is_empty() {
            return Vec::new();
        }

        let selected = shortlist_documents(query, documents, self.max_options);
        let mut criteria = serde_json::Map::new();
        for (rank, index) in selected.iter().enumerate() {
            criteria.insert(
                format!("option-{rank}"),
                Value::String(documents[*index].to_string()),
            );
        }

        let mut request_body = json!({
            "state": query,
            "questions": {
                "tool": {
                    "type": "choice",
                    "instructions": "Which candidate tool best satisfies the task? Choose the closest match based on capability, not availability.",
                    "criteria": criteria
                }
            }
        });
        if let Some(model) = &self.model {
            request_body["model"] = Value::String(model.clone());
        }

        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build();
        let mut request = agent
            .post(&self.endpoint)
            .set("Content-Type", "application/json");
        if let Some(key) = &self.api_key {
            request = request.set("Authorization", &format!("Bearer {key}"));
        }
        let Ok(response) = request.send_json(&request_body) else {
            return StatisticalScorer.score_batch(query, documents);
        };
        let Ok(payload) = response.into_json::<Value>() else {
            return StatisticalScorer.score_batch(query, documents);
        };

        let answer = payload
            .get("answers")
            .and_then(|answers| answers.get("tool"));
        let mut scores = vec![0.0; documents.len()];
        if let Some(probabilities) = answer
            .and_then(|value| value.get("probabilities"))
            .and_then(Value::as_object)
        {
            for (rank, index) in selected.iter().enumerate() {
                if let Some(score) = probabilities
                    .get(&format!("option-{rank}"))
                    .and_then(Value::as_f64)
                {
                    scores[*index] = (score as f32).clamp(0.0, 1.0);
                }
            }
        } else {
            return StatisticalScorer.score_batch(query, documents);
        }
        scores
    }
}

fn shortlist_documents(query: &str, documents: &[&str], max_options: usize) -> Vec<usize> {
    if documents.len() <= max_options {
        return (0..documents.len()).collect();
    }
    let baseline = StatisticalScorer.score_batch(query, documents);
    let mut indices: Vec<usize> = (0..documents.len()).collect();
    indices.sort_by(|left, right| {
        baseline[*right]
            .partial_cmp(&baseline[*left])
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| left.cmp(right))
    });
    indices.truncate(max_options);
    indices
}

#[derive(Serialize)]
struct RerankRequest<'a> {
    model: &'a str,
    query: &'a str,
    documents: &'a [&'a str],
    top_n: usize,
}

#[derive(Deserialize)]
struct RerankResponseItem {
    index: usize,
    relevance_score: f32,
}

#[derive(Deserialize)]
struct RerankResponse {
    results: Vec<RerankResponseItem>,
}

impl HttpRerankScorer {
    pub fn new(endpoint: String, api_key: Option<String>, model: String) -> Self {
        Self {
            endpoint,
            api_key,
            model,
        }
    }
}

impl SemanticScorer for HttpRerankScorer {
    fn name(&self) -> &'static str {
        "Neural Reranker (Jina/API)"
    }

    fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32> {
        if documents.is_empty() {
            return Vec::new();
        }

        let mut scores = vec![0.0f32; documents.len()];
        let req_body = RerankRequest {
            model: &self.model,
            query,
            documents,
            top_n: documents.len(),
        };

        let mut request = ureq::post(&self.endpoint).set("Content-Type", "application/json");

        if let Some(key) = &self.api_key {
            request = request.set("Authorization", &format!("Bearer {key}"));
        }

        let response = match request.send_json(&req_body) {
            Ok(resp) => resp,
            Err(_) => return scores,
        };

        if let Ok(rerank_data) = response.into_json::<RerankResponse>() {
            for item in rerank_data.results {
                if item.index < scores.len() {
                    scores[item.index] = item.relevance_score.clamp(0.0, 1.0);
                }
            }
        }

        scores
    }
}

/// External process / CLI command scorer for plugging in local HuggingFace / PyTorch models.
pub struct CommandScorer {
    pub command: String,
}

#[derive(Serialize)]
struct CommandInput<'a> {
    query: &'a str,
    documents: &'a [&'a str],
}

#[derive(Deserialize)]
struct CommandOutput {
    scores: Vec<f32>,
}

impl SemanticScorer for CommandScorer {
    fn name(&self) -> &'static str {
        "Local Script Model"
    }

    fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32> {
        let mut scores = vec![0.0f32; documents.len()];
        let Ok(mut child) = Command::new(&self.command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        else {
            return scores;
        };

        let payload = CommandInput { query, documents };
        if let Some(ref mut stdin) = child.stdin
            && let Ok(json_bytes) = serde_json::to_vec(&payload)
        {
            let _ = stdin.write_all(&json_bytes);
        }

        if let Ok(output) = child.wait_with_output()
            && output.status.success()
            && let Ok(parsed) = serde_json::from_slice::<CommandOutput>(&output.stdout)
        {
            for (i, s) in parsed.scores.into_iter().enumerate() {
                if i < scores.len() {
                    scores[i] = s.clamp(0.0, 1.0);
                }
            }
        }

        scores
    }
}

/// Pure mathematical n-gram and token frequency statistical scorer (zero-hardcoding fallback).
pub struct StatisticalScorer;

impl SemanticScorer for StatisticalScorer {
    fn name(&self) -> &'static str {
        "Statistical Token & N-gram Model"
    }

    fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32> {
        let query_ngrams = extract_ngrams(query);
        let query_tokens: BTreeSet<String> = query
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() >= 2)
            .map(|w| w.to_lowercase())
            .collect();

        if query_ngrams.is_empty() && query_tokens.is_empty() {
            return vec![0.0; documents.len()];
        }

        documents
            .iter()
            .map(|doc| {
                let doc_lower = doc.to_lowercase();
                let doc_tokens: BTreeSet<String> = doc_lower
                    .split(|c: char| !c.is_alphanumeric())
                    .filter(|w| w.len() >= 2)
                    .map(|w| w.to_string())
                    .collect();
                let doc_ngrams = extract_ngrams(&doc_lower);

                // Token overlap
                let token_intersect = query_tokens.intersection(&doc_tokens).count();
                let token_union = query_tokens.union(&doc_tokens).count();
                let has_phrase = doc_lower.contains(&query.to_lowercase());

                // If zero tokens match and no phrase match, relevance is zero
                if token_intersect == 0 && !has_phrase {
                    return 0.0;
                }

                let token_coverage = if query_tokens.is_empty() {
                    0.0
                } else {
                    token_intersect as f32 / query_tokens.len() as f32
                };

                let token_jaccard = if token_union == 0 {
                    0.0
                } else {
                    token_intersect as f32 / token_union as f32
                };

                // Character bigram overlap (supports multilingual & CJK directly)
                let ngram_intersect = query_ngrams.intersection(&doc_ngrams).count();
                let ngram_sim = if query_ngrams.is_empty() {
                    0.0
                } else {
                    ngram_intersect as f32 / query_ngrams.len() as f32
                };

                let phrase_bonus = if has_phrase { 0.25 } else { 0.0 };

                (token_coverage * 0.55 + token_jaccard * 0.20 + ngram_sim * 0.25 + phrase_bonus)
                    .clamp(0.0, 1.0)
            })
            .collect()
    }
}

fn extract_ngrams(text: &str) -> BTreeSet<String> {
    let mut ngrams = BTreeSet::new();
    let chars: Vec<char> = text
        .chars()
        .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if chars.len() >= 2 {
        for window in chars.windows(2) {
            ngrams.insert(window.iter().collect::<String>());
        }
    }
    if chars.len() >= 3 {
        for window in chars.windows(3) {
            ngrams.insert(window.iter().collect::<String>());
        }
    }
    ngrams
}

/// Dynamic manager choosing the best available semantic model (HTTP neural model, local script, or statistical fallback).
pub struct SemanticMatcher {
    active_scorer: Box<dyn SemanticScorer>,
}

impl Default for SemanticMatcher {
    fn default() -> Self {
        Self::auto_detect(None, None, None)
    }
}

impl SemanticMatcher {
    /// Auto-detect semantic model from explicit options or environment variables.
    pub fn auto_detect(
        rerank_url: Option<String>,
        api_key: Option<String>,
        model_cmd: Option<String>,
    ) -> Self {
        if let Some(cmd) = model_cmd.or_else(|| std::env::var("RUNBOOK_MODEL_CMD").ok()) {
            return Self {
                active_scorer: Box::new(CommandScorer { command: cmd }),
            };
        }

        let system_one_url = std::env::var("RUNBOOK_SYSTEM_ONE_URL").ok();
        if let Some(endpoint) = system_one_url {
            let model = std::env::var("RUNBOOK_SYSTEM_ONE_MODEL").ok();
            let max_options = std::env::var("RUNBOOK_SYSTEM_ONE_MAX_OPTIONS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(32);
            return Self {
                active_scorer: Box::new(SystemOneScorer::new(
                    endpoint,
                    api_key
                        .clone()
                        .or_else(|| std::env::var("RUNBOOK_SYSTEM_ONE_API_KEY").ok()),
                    model,
                    max_options,
                )),
            };
        }

        let api_key = api_key
            .or_else(|| std::env::var("JINA_API_KEY").ok())
            .or_else(|| std::env::var("RUNBOOK_MODEL_API_KEY").ok());

        let url = rerank_url
            .or_else(|| std::env::var("RUNBOOK_RERANK_URL").ok())
            .or_else(|| {
                if api_key.is_some() {
                    Some("https://api.jina.ai/v1/rerank".to_string())
                } else {
                    None
                }
            });

        if let Some(endpoint) = url {
            let model = std::env::var("RUNBOOK_RERANK_MODEL")
                .unwrap_or_else(|_| "jina-reranker-v2-base-multilingual".to_string());
            Self {
                active_scorer: Box::new(HttpRerankScorer::new(endpoint, api_key, model)),
            }
        } else {
            Self {
                active_scorer: Box::new(StatisticalScorer),
            }
        }
    }

    pub fn scorer_name(&self) -> &'static str {
        self.active_scorer.name()
    }

    pub fn score_batch(&self, query: &str, documents: &[&str]) -> Vec<f32> {
        self.active_scorer.score_batch(query, documents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_one_shortlists_large_candidate_sets() {
        let documents = ["pdf parser", "rust test runner", "git client"];
        let selected = shortlist_documents("run rust tests", &documents, 2);
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0], 1);
    }

    #[test]
    fn statistical_scorer_computes_character_ngrams_without_hardcoding() {
        let scorer = StatisticalScorer;
        let docs = [
            "docling: IBM document parsing CLI converting PDFs, tables, and complex formats into LLM-ready markdown.",
            "git: Distributed version control system for tracking changes in source code.",
        ];

        let scores = scorer.score_batch("parse pdf tables to markdown", &docs);
        assert_eq!(scores.len(), 2);
        assert!(
            scores[0] > scores[1],
            "docling should score higher than git for pdf parsing"
        );
    }

    #[test]
    fn statistical_scorer_supports_multilingual_natively() {
        let scorer = StatisticalScorer;
        let docs = [
            "抽取 pdf 表格与数学公式，生成 markdown 文档格式",
            "代码版本管理工具，记录提交与合并历史",
        ];

        let scores = scorer.score_batch("提取 pdf 表格", &docs);
        assert!(
            scores[0] > scores[1],
            "PDF document tool should match Chinese query"
        );
    }
}
