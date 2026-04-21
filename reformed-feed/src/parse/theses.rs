use anyhow::Result;
use doc_feed::document::ParseStrategy;

use crate::documents::theses::{ThesesDoc, ThesisItem};

use super::parse_yaml;

pub struct ByThesis {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByThesis {
    type Doc = ThesesDoc;

    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        let yaml = parse_yaml(data)?;
        let doc_title = yaml["name"]
            .as_str()
            .unwrap_or(&self.doc_title)
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
                    doc_id: self.doc_id.clone(),
                    doc_title: doc_title.clone(),
                    number: (i + 1) as u32,
                    text,
                })
            })
            .collect();

        Ok(ThesesDoc {
            id: self.doc_id.clone(),
            title: doc_title,
            items,
        })
    }
}
