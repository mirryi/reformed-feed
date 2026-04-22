pub mod articles;
pub mod canon;
pub mod catechism;
pub mod confession;
pub mod creed;
pub mod theses;

use anyhow::Result;

/// Helper: parse raw JSON bytes into a serde_json::Value.
pub fn parse_json(data: &[u8]) -> Result<serde_json::Value> {
    let json = serde_json::from_slice(data)?;
    Ok(json)
}

/// Helper: parse raw YAML bytes into a serde_yaml::Value.
pub fn parse_yaml(data: &[u8]) -> Result<serde_yaml::Value> {
    let yaml = serde_yaml::from_slice(data)?;
    Ok(yaml)
}

/// Helper: get content, preferring ContentWithProofs over Content.
pub fn get_content(entry: &serde_json::Value) -> String {
    entry["ContentWithProofs"]
        .as_str()
        .or_else(|| entry["Content"].as_str())
        .unwrap_or("")
        .to_string()
}
