use serde_json::{Value, json};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use crate::commands::category::{CategoryCommand, query_category};
use crate::commands::prefer::{PreferCommand, run_prefer};
use crate::commands::recommend::{RecommendCommand, query_recommend};
use crate::commands::scan::{ScanCommand, scan};
use crate::model::{
    CategoryInput, PreferAction, PreferInput, RecommendInput, ScanInput, ScanMode, ToolPreference,
};

/// Run the MCP server over standard input and output using newline-delimited JSON-RPC.
pub fn run_stdio_server() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        if let Ok(request) = serde_json::from_str::<Value>(&line)
            && let Some(response) = handle_message(&request)
        {
            let serialized = serde_json::to_string(&response)?;
            writeln!(out, "{serialized}")?;
            out.flush()?;
        }
    }

    Ok(())
}

/// Process a single JSON-RPC message according to the Model Context Protocol.
pub fn handle_message(msg: &Value) -> Option<Value> {
    let id = msg.get("id");
    let method = msg.get("method").and_then(Value::as_str)?;

    match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "agent-runbook",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        })),
        "notifications/initialized" => {
            // Notification: No response
            None
        }
        "ping" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {}
        })),
        "tools/list" => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "runbook_scan",
                        "description": "Scan machine-level tools, current project requirements, and recommended guardrails for AI coding agents.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "cwd": {
                                    "type": "string",
                                    "description": "Directory to scan. Defaults to current directory."
                                },
                                "mode": {
                                    "type": "string",
                                    "enum": ["all", "global", "local"],
                                    "description": "Scan mode: all (default), global, or local."
                                },
                                "minimal": {
                                    "type": "boolean",
                                    "description": "Output minimal tool name inventory only."
                                }
                            }
                        }
                    },
                    {
                        "name": "runbook_recommend",
                        "description": "Recommend best command-line tools for a natural language task description with model-driven semantic scoring and Agent-Ready quality ratings.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "task": {
                                    "type": "string",
                                    "description": "Natural language task requirement (e.g. 'parse pdf tables into markdown', 'audit dependencies for security')."
                                },
                                "lang": {
                                    "type": "string",
                                    "description": "Filter by programming language or file ecosystem (e.g. 'rust', 'python', 'all')."
                                },
                                "platform": {
                                    "type": "string",
                                    "description": "Filter by target platform (e.g. 'web', 'mobile', 'local', 'cloud')."
                                },
                                "profile": {
                                    "type": "string",
                                    "enum": ["balanced", "safety", "automation", "minimal"],
                                    "description": "Scoring weight profile."
                                },
                                "limit": {
                                    "type": "integer",
                                    "description": "Maximum number of tools to return (default 5)."
                                }
                            },
                            "required": ["task"]
                        }
                    },
                    {
                        "name": "runbook_category",
                        "description": "List functional tool categories or inspect candidate tools for specific categories and languages.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "categories": {
                                    "type": "array",
                                    "items": { "type": "string" },
                                    "description": "Tool categories to inspect. Empty to list categories."
                                },
                                "lang": {
                                    "type": "string",
                                    "description": "Filter tools by language."
                                },
                                "platform": {
                                    "type": "string",
                                    "description": "Filter tools by platform."
                                },
                                "score": {
                                    "type": "boolean",
                                    "description": "Calculate and display Agent-Ready Scores."
                                },
                                "profile": {
                                    "type": "string",
                                    "enum": ["balanced", "safety", "automation", "minimal"],
                                    "description": "Scoring profile."
                                }
                            }
                        }
                    },
                    {
                        "name": "runbook_prefer",
                        "description": "Query or record explicit repository-local tool preferences (e.g. set preferred test runner, linter, or shell).",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "action": {
                                    "type": "string",
                                    "enum": ["list", "set", "unset"],
                                    "description": "Action: list, set, or unset."
                                },
                                "category": {
                                    "type": "string",
                                    "description": "Tool category for set/unset."
                                },
                                "lang": {
                                    "type": "string",
                                    "description": "Language key for set/unset."
                                },
                                "tool": {
                                    "type": "string",
                                    "description": "Tool name for set."
                                },
                                "reason": {
                                    "type": "string",
                                    "description": "Reason for setting the tool preference."
                                }
                            },
                            "required": ["action"]
                        }
                    }
                ]
            }
        })),
        "tools/call" => {
            let params = msg.get("params")?;
            let tool_name = params.get("name").and_then(Value::as_str)?;
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));

            let result_text = execute_tool_call(tool_name, &args);

            Some(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": result_text
                        }
                    ]
                }
            }))
        }
        _ => Some(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": -32601,
                "message": format!("Method '{method}' not found")
            }
        })),
    }
}

fn execute_tool_call(name: &str, args: &Value) -> String {
    let cwd = args
        .get("cwd")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    match name {
        "runbook_scan" => {
            let mode = match args.get("mode").and_then(Value::as_str) {
                Some("global") => ScanMode::Global,
                Some("local") => ScanMode::Local,
                _ => ScanMode::All,
            };
            let minimal = args
                .get("minimal")
                .and_then(Value::as_bool)
                .unwrap_or(false);

            let res = scan(ScanCommand {
                input: ScanInput { cwd, mode, minimal },
            });
            serde_json::to_string_pretty(&res).unwrap_or_else(|e| format!(r#"{{"error": "{e}"}}"#))
        }
        "runbook_recommend" => {
            let task_str = args.get("task").and_then(Value::as_str).unwrap_or("");
            let lang = args.get("lang").and_then(Value::as_str).map(String::from);
            let platform = args
                .get("platform")
                .and_then(Value::as_str)
                .map(String::from);
            let profile = args
                .get("profile")
                .and_then(Value::as_str)
                .map(String::from);
            let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(5) as usize;

            let task: Vec<String> = task_str.split_whitespace().map(String::from).collect();

            match query_recommend(RecommendCommand {
                input: RecommendInput {
                    cwd,
                    task,
                    lang,
                    platform,
                    profile,
                    rerank_url: None,
                    api_key: None,
                    model_cmd: None,
                    limit,
                },
            }) {
                Ok(res) => serde_json::to_string_pretty(&res)
                    .unwrap_or_else(|e| format!(r#"{{"error": "{e}"}}"#)),
                Err(err) => format!(r#"{{"error": "{err}"}}"#),
            }
        }
        "runbook_category" => {
            let categories: Vec<String> = args
                .get("categories")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();
            let lang = args.get("lang").and_then(Value::as_str).map(String::from);
            let platform = args
                .get("platform")
                .and_then(Value::as_str)
                .map(String::from);
            let score = args.get("score").and_then(Value::as_bool).unwrap_or(false);
            let profile = args
                .get("profile")
                .and_then(Value::as_str)
                .map(String::from);

            match query_category(CategoryCommand {
                input: CategoryInput {
                    cwd,
                    categories,
                    lang,
                    platform,
                    score,
                    profile,
                },
            }) {
                Ok(res) => serde_json::to_string_pretty(&res)
                    .unwrap_or_else(|e| format!(r#"{{"error": "{e}"}}"#)),
                Err(err) => format!(r#"{{"error": "{err}"}}"#),
            }
        }
        "runbook_prefer" => {
            let action_type = args.get("action").and_then(Value::as_str).unwrap_or("list");
            let action = match action_type {
                "set" => {
                    let category = args
                        .get("category")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let lang = args
                        .get("lang")
                        .and_then(Value::as_str)
                        .unwrap_or("all")
                        .to_string();
                    let tool = args
                        .get("tool")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let reason = args
                        .get("reason")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    PreferAction::Set(ToolPreference {
                        category,
                        lang,
                        tool,
                        reason,
                    })
                }
                "unset" => {
                    let category = args
                        .get("category")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    let lang = args
                        .get("lang")
                        .and_then(Value::as_str)
                        .unwrap_or("all")
                        .to_string();
                    PreferAction::Unset { category, lang }
                }
                _ => PreferAction::List,
            };

            match run_prefer(PreferCommand {
                input: PreferInput { cwd, action },
            }) {
                Ok(res) => serde_json::to_string_pretty(&res)
                    .unwrap_or_else(|e| format!(r#"{{"error": "{e}"}}"#)),
                Err(err) => format!(r#"{{"error": "{err}"}}"#),
            }
        }
        _ => format!(r#"{{"error": "Unknown tool '{name}'"}}"#),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_handles_initialize_lifecycle() {
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05"
            }
        });

        let resp = handle_message(&req).expect("expected initialize response");
        assert_eq!(resp["id"], 1);
        assert_eq!(resp["result"]["protocolVersion"], "2024-11-05");
        assert_eq!(resp["result"]["serverInfo"]["name"], "agent-runbook");
    }

    #[test]
    fn mcp_lists_available_tools() {
        let req = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list"
        });

        let resp = handle_message(&req).expect("expected tools/list response");
        let tools = resp["result"]["tools"].as_array().expect("tools array");
        assert_eq!(tools.len(), 4);
        assert!(tools.iter().any(|t| t["name"] == "runbook_scan"));
        assert!(tools.iter().any(|t| t["name"] == "runbook_recommend"));
        assert!(tools.iter().any(|t| t["name"] == "runbook_category"));
        assert!(tools.iter().any(|t| t["name"] == "runbook_prefer"));
    }

    #[test]
    fn mcp_executes_recommend_tool_call() {
        let req = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "runbook_recommend",
                "arguments": {
                    "task": "parse pdf tables to markdown",
                    "limit": 2
                }
            }
        });

        let resp = handle_message(&req).expect("expected tools/call response");
        assert_eq!(resp["id"], 3);
        let text = resp["result"]["content"][0]["text"].as_str().expect("text");
        assert!(text.contains("docling") || text.contains("marker") || text.contains("tools"));
    }
}
