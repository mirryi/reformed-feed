use anyhow::Result;
use doc_feed::document::ParseStrategy;

use crate::documents::canon::{CanonDoc, CanonItem};
use crate::documents::parse_proofs;

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

pub struct ByArticle {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByArticle {
    type Doc = CanonDoc;

    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
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
                    doc_id: self.doc_id.clone(),
                    doc_title: self.doc_title.clone(),
                    article_number,
                    article_title,
                    content,
                    content_with_proofs,
                    proofs,
                }
            })
            .collect();

        Ok(CanonDoc {
            id: self.doc_id.clone(),
            title: self.doc_title.clone(),
            items,
        })
    }
}
