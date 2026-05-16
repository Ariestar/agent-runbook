use crate::model::ToolSpec;

pub fn tool_registry() -> Vec<ToolSpec> {
    serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/tools.json")))
        .expect("embedded tool index must be valid JSON")
}
