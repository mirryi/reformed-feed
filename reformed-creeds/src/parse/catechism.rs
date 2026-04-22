use anyhow::Result;

use crate::types::catechism::{CatechismDoc, CatechismItem};
use crate::types::parse_proofs;

use super::parse_json;

/// Parse a catechism with one item per Q&A pair.
pub fn parse_by_question(id: &str, title: &str, data: &[u8]) -> Result<CatechismDoc> {
    let json = parse_json(data)?;
    let entries = json["Data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Catechism Data is not an array"))?;

    let items = entries
        .iter()
        .map(|entry| {
            let number = entry["Number"]
                .as_u64()
                .map(|n| n.to_string())
                .or_else(|| entry["Number"].as_str().map(String::from))
                .unwrap_or_else(|| "?".to_string());
            let question = entry["Question"].as_str().unwrap_or("").to_string();
            let answer = entry["Answer"].as_str().unwrap_or("").to_string();
            let answer_with_proofs = entry["AnswerWithProofs"].as_str().map(String::from);
            let proofs = parse_proofs(entry);

            CatechismItem {
                doc_id: id.to_string(),
                doc_title: title.to_string(),
                number,
                question,
                answer,
                answer_with_proofs,
                proofs,
            }
        })
        .collect();

    Ok(CatechismDoc {
        id: id.to_string(),
        title: title.to_string(),
        items,
    })
}
