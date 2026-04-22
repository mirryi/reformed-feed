use anyhow::Result;

use crate::types::articles::{ArticleItem, ArticlesDoc};

use super::parse_yaml;

pub fn parse_by_article(id: &str, title: &str, data: &[u8]) -> Result<ArticlesDoc> {
    let yaml = parse_yaml(data)?;
    let doc_title = yaml["name"]
        .as_str()
        .unwrap_or(title)
        .to_string();

    let chapters = yaml["chapters"]
        .as_sequence()
        .ok_or_else(|| anyhow::anyhow!("Articles chapters is not a sequence"))?;

    let items = chapters
        .iter()
        .filter_map(|ch| {
            let number = ch["number"].as_u64()? as u32;
            let name = ch["name"].as_str()?.to_string();
            let text = ch["text"].as_str()?.to_string();
            Some(ArticleItem {
                doc_id: id.to_string(),
                doc_title: doc_title.clone(),
                number,
                name,
                text,
            })
        })
        .collect();

    Ok(ArticlesDoc {
        id: id.to_string(),
        title: doc_title,
        items,
    })
}
