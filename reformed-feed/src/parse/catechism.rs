use anyhow::Result;
use doc_feed::document::ParseStrategy;

use crate::documents::catechism::{CatechismDoc, CatechismItem};
use crate::documents::parse_proofs;

use super::parse_json;

/// Parse a catechism with one item per Q&A pair.
pub struct ByQuestion {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByQuestion {
    type Doc = CatechismDoc;

    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
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
                    doc_id: self.doc_id.clone(),
                    doc_title: self.doc_title.clone(),
                    number,
                    question,
                    answer,
                    answer_with_proofs,
                    proofs,
                }
            })
            .collect();

        Ok(CatechismDoc {
            id: self.doc_id.clone(),
            title: self.doc_title.clone(),
            items,
        })
    }
}
