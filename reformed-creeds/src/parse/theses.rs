use anyhow::Result;

use crate::types::theses::{ThesesDoc, ThesisItem};

use super::parse_yaml;

pub fn parse_by_thesis(id: &str, title: &str, data: &[u8]) -> Result<ThesesDoc> {
    let yaml = parse_yaml(data)?;
    let doc_title = yaml["name"]
        .as_str()
        .unwrap_or(title)
        .to_string();

    let chapters = yaml["chapters"]
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("Theses chapters is not a sequence"))?;

    let items = chapters
        .iter()
        .enumerate()
        .filter_map(|(i, thesis)| {
            let text = thesis.as_str()?.to_string();
            Some(ThesisItem {
                doc_id: id.to_string(),
                doc_title: doc_title.clone(),
                number: (i + 1) as u32,
                text,
            })
        })
        .collect();

    Ok(ThesesDoc {
        id: id.to_string(),
        title: doc_title,
        items,
    })
}
