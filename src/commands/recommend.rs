use crate::discovery::command::CommandIndex;
use crate::error::Result;
use crate::matching::SemanticMatcher;
use crate::model::{
    Availability, RecommendInput, RecommendResult, RecommendedTool, ToolCandidate, ToolSpec,
};
use crate::preferences::load_preferences;
use crate::registry::tool_registry;
use crate::scoring::{ScoreContext, ScoringPipeline};

pub struct RecommendCommand {
    pub input: RecommendInput,
}

pub fn query_recommend(command: RecommendCommand) -> Result<RecommendResult> {
    let query = command.input.task.join(" ");
    let registry = tool_registry();
    let preferences = load_preferences(&command.input.cwd)?;
    let command_index = CommandIndex::new();

    // 1. Filter tools by language and platform constraints
    let filtered_tools: Vec<&ToolSpec> = registry
        .iter()
        .filter(|t| {
            command
                .input
                .lang
                .as_deref()
                .is_none_or(|l| t.supports_lang(l))
        })
        .filter(|t| {
            command
                .input
                .platform
                .as_deref()
                .is_none_or(|p| t.supports_platform(p))
        })
        .collect();

    if filtered_tools.is_empty() || query.trim().is_empty() {
        return Ok(RecommendResult {
            query,
            profile: command
                .input
                .profile
                .unwrap_or_else(|| "balanced".to_string()),
            model_name: "none".to_string(),
            tools: Vec::new(),
        });
    }

    // 2. Prepare candidate document text representations for model scoring
    let document_texts: Vec<String> = filtered_tools
        .iter()
        .map(|t| {
            let use_cases = if t.use_when.is_empty() {
                String::new()
            } else {
                format!(" Best for: {}.", t.use_when.join("; "))
            };
            format!("{}: {}{}", t.name, t.summary, use_cases)
        })
        .collect();

    let doc_slices: Vec<&str> = document_texts.iter().map(|s| s.as_str()).collect();

    // 3. Initialize model-driven semantic matcher
    let matcher = SemanticMatcher::auto_detect(
        command.input.rerank_url,
        command.input.api_key,
        command.input.model_cmd,
    );
    let model_name = matcher.scorer_name().to_string();

    // 4. Batch query the model for relevance scores
    let relevance_scores = matcher.score_batch(&query, &doc_slices);

    // 5. Initialize pluggable Agent-Ready Score pipeline
    let profile_name = command
        .input
        .profile
        .unwrap_or_else(|| "balanced".to_string());
    let scoring_pipeline = ScoringPipeline::from_profile_name(Some(&profile_name));

    // 6. Assemble candidate data, compute ARS, and fuse scores
    let mut candidates: Vec<RecommendedTool> = Vec::with_capacity(filtered_tools.len());

    for (i, tool) in filtered_tools.iter().enumerate() {
        let rel_float = relevance_scores.get(i).copied().unwrap_or(0.0);
        let relevance_score = (rel_float * 100.0).round() as u32;

        // Skip irrelevant tools (below 20% relevance threshold)
        if relevance_score < 20 && query.len() > 3 {
            continue;
        }

        let primary_cat = tool
            .category
            .first()
            .map(|s| s.as_str())
            .unwrap_or_default();
        let candidate_helper = ToolCandidate::build(
            tool,
            &preferences,
            &command_index,
            primary_cat,
            command.input.lang.as_deref(),
        );

        let context = ScoreContext {
            lang: command.input.lang.as_deref(),
            category: Some(primary_cat),
            platform: command.input.platform.as_deref(),
        };

        let agent_score = scoring_pipeline.score_candidate(&candidate_helper, &context);

        // Fused scoring calculation:
        // 60% Task Semantic Relevance + 25% Agent-Ready Quality + 15% Local Availability + Preference Bonus
        let avail_weight = match &candidate_helper.availability {
            Availability::Found { .. } => 100.0,
            Availability::Missing { .. } => 30.0,
        };

        let pref_bonus = if candidate_helper.preference.is_some() && relevance_score >= 35 {
            10.0
        } else {
            0.0
        };

        let fused_raw = (relevance_score as f32 * 0.60)
            + (agent_score.total as f32 * 0.25)
            + (avail_weight * 0.15)
            + pref_bonus;

        let fused_score = (fused_raw.round() as u32).min(100);

        let match_reason = if relevance_score >= 80 {
            "high semantic alignment with task intent".to_string()
        } else if relevance_score >= 40 {
            "moderate semantic alignment with capabilities".to_string()
        } else {
            "broad domain match".to_string()
        };

        candidates.push(RecommendedTool {
            name: tool.name.clone(),
            binary: tool.binary.clone(),
            summary: tool.summary.clone(),
            category: tool.category.clone(),
            lang: tool.lang.clone(),
            docs: if tool.docs.is_empty() {
                tool.homepage.clone()
            } else {
                tool.docs.clone()
            },
            homepage: tool.homepage.clone(),
            availability: candidate_helper.availability,
            preference: candidate_helper.preference,
            relevance_score,
            match_reason,
            agent_score,
            fused_score,
        });
    }

    // 7. Sort by fused score descending
    candidates.sort_by(|a, b| {
        if a.fused_score != b.fused_score {
            return b.fused_score.cmp(&a.fused_score);
        }

        b.relevance_score.cmp(&a.relevance_score)
    });

    candidates.truncate(command.input.limit);

    Ok(RecommendResult {
        query,
        profile: profile_name,
        model_name,
        tools: candidates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recommend_returns_relevant_tools_for_query() {
        let result = query_recommend(RecommendCommand {
            input: RecommendInput {
                cwd: std::env::current_dir().unwrap(),
                task: vec![
                    "parse".to_string(),
                    "pdf".to_string(),
                    "markdown".to_string(),
                ],
                lang: None,
                platform: None,
                profile: None,
                rerank_url: None,
                api_key: None,
                model_cmd: None,
                limit: 3,
            },
        })
        .unwrap();

        assert!(!result.tools.is_empty());
        assert!(
            result
                .tools
                .iter()
                .any(|t| t.name == "docling" || t.name == "marker" || t.name == "pdftotext")
        );
    }
}
