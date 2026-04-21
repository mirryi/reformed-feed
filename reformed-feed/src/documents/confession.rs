use doc_feed::document::{Document, IntoFeedEntry};

use super::{format_proofs, html_escape, Proof};

#[derive(Debug, Clone)]
pub struct ConfessionItem {
    pub doc_id: String,
    pub doc_title: String,
    pub chapter_number: String,
    pub chapter_title: String,
    pub section_number: String,
    pub content: String,
    pub content_with_proofs: Option<String>,
    pub proofs: Vec<Proof>,
}

impl IntoFeedEntry for ConfessionItem {
    fn title(&self) -> String {
        format!(
            "{} Ch.{} Sec.{}",
            self.doc_title, self.chapter_number, self.section_number
        )
    }

    fn description(&self) -> String {
        let body = format!(
            "Chapter {}: {}\nSection {}\n\n{}",
            self.chapter_number, self.chapter_title, self.section_number, self.content
        );
        let mut html = format!(
            "<div style='white-space: pre-wrap;'>{}</div>",
            html_escape(&body)
        );
        if let Some(refs) = format_proofs(&self.proofs) {
            html.push_str(&format!(
                "<hr/><div style='white-space: pre-wrap; color: #666;'>{}</div>",
                html_escape(&refs)
            ));
        }
        html
    }

    fn guid(&self) -> String {
        format!(
            "{}:ch{}:sec{}",
            self.doc_id, self.chapter_number, self.section_number
        )
    }
}

#[derive(Debug, Clone)]
pub struct ConfessionDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ConfessionItem>,
}

impl Document for ConfessionDoc {
    type Item = ConfessionItem;
    fn items(&self) -> Vec<Self::Item> {
        self.items.clone()
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> &str {
        &self.title
    }
}
