use anyhow::Result;

use crate::types::creed::CreedDoc;

use super::parse_json;

pub fn parse_whole(id: &str, title: &str, data: &[u8]) -> Result<CreedDoc> {
    let json = parse_json(data)?;
    let content = json["Data"]["Content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(CreedDoc {
        id: id.to_string(),
        title: title.to_string(),
        content,
    })
}
