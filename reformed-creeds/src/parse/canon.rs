use anyhow::Result;

use crate::types::canon::{CanonDoc, CanonItem};
use crate::types::parse_proofs;

use super::{get_content, parse_json};

/// Helper to extract a string-or-int field as String.
fn str_or_int(value: &serde_json::Value) -> String {
    value
        .as_str()
        .map(String::from)
        .or_else(|| value.as_u64().map(|n| n.to_string()))
        .or_else(|| value.as_i64().map(|n| n.to_string()))
        .unwrap_or_else(|| "?".to_string())
}

pub fn parse_by_article(id: &str, title: &str, data: &[u8]) -> Result<CanonDoc> {
    let json = parse_json(data)?;
    let entries = json["Data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Canon Data is not an array"))?;

    let items = entries
        .iter()
        .map(|entry| {
            let article_number = str_or_int(&entry["Article"]);
            let article_title = entry["Title"].as_str().unwrap_or("").to_string();
            let content = get_content(entry);
            let content_with_proofs = entry["ContentWithProofs"].as_str().map(String::from);
            let proofs = parse_proofs(entry);

            CanonItem {
                doc_id: id.to_string(),
                doc_title: title.to_string(),
                article_number,
                article_title,
                content,
                content_with_proofs,
                proofs,
            }
        })
        .collect();

    Ok(CanonDoc {
        id: id.to_string(),
        title: title.to_string(),
        items,
    })
}
