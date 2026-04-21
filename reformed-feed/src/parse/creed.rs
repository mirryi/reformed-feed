use anyhow::Result;
use doc_feed::document::ParseStrategy;

use crate::documents::creed::CreedDoc;

use super::parse_json;

pub struct WholeCreed {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for WholeCreed {
    type Doc = CreedDoc;

    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        let json = parse_json(data)?;
        let content = json["Data"]["Content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(CreedDoc {
            id: self.doc_id.clone(),
            title: self.doc_title.clone(),
            content,
        })
    }
}
