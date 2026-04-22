use anyhow::Result;

use crate::types::confession::{ConfessionDoc, ConfessionItem};
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

/// Parse a confession with one item per section.
pub fn parse_by_section(id: &str, title: &str, data: &[u8]) -> Result<ConfessionDoc> {
    let json = parse_json(data)?;
    let chapters = json["Data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Confession Data is not an array"))?;

    let mut items = Vec::new();
    for chapter in chapters {
        let chapter_number = str_or_int(&chapter["Chapter"]);
        let chapter_title = chapter["Title"].as_str().unwrap_or("").to_string();

        let sections = chapter["Sections"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Sections is not an array"))?;

        for section in sections {
            let section_number = str_or_int(&section["Section"]);
            let content = get_content(section);
            let content_with_proofs = section["ContentWithProofs"].as_str().map(String::from);
            let proofs = parse_proofs(section);

            items.push(ConfessionItem {
                doc_id: id.to_string(),
                doc_title: title.to_string(),
                chapter_number: chapter_number.clone(),
                chapter_title: chapter_title.clone(),
                section_number,
                content,
                content_with_proofs,
                proofs,
            });
        }
    }

    Ok(ConfessionDoc {
        id: id.to_string(),
        title: title.to_string(),
        items,
    })
}

/// Parse a confession with one item per chapter (all sections concatenated).
pub fn parse_by_chapter(id: &str, title: &str, data: &[u8]) -> Result<ConfessionDoc> {
    let json = parse_json(data)?;
    let chapters = json["Data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Confession Data is not an array"))?;

    let mut items = Vec::new();
    for chapter in chapters {
        let chapter_number = str_or_int(&chapter["Chapter"]);
        let chapter_title = chapter["Title"].as_str().unwrap_or("").to_string();

        let sections = chapter["Sections"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Sections is not an array"))?;

        let mut all_content = Vec::new();
        let mut all_proofs = Vec::new();
        for section in sections {
            all_content.push(get_content(section));
            all_proofs.extend(parse_proofs(section));
        }

        items.push(ConfessionItem {
            doc_id: id.to_string(),
            doc_title: title.to_string(),
            chapter_number: chapter_number.clone(),
            chapter_title: chapter_title.clone(),
            section_number: "all".to_string(),
            content: all_content.join("\n\n"),
            content_with_proofs: None,
            proofs: all_proofs,
        });
    }

    Ok(ConfessionDoc {
        id: id.to_string(),
        title: title.to_string(),
        items,
    })
}
