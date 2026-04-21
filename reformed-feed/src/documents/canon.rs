use doc_feed::document::{Document, IntoFeedEntry};

use super::{format_proofs, html_escape, Proof};

#[derive(Debug, Clone)]
pub struct CanonItem {
    pub doc_id: String,
    pub doc_title: String,
    pub article_number: String,
    pub article_title: String,
    pub content: String,
    pub content_with_proofs: Option<String>,
    pub proofs: Vec<Proof>,
}

impl IntoFeedEntry for CanonItem {
    fn title(&self) -> String {
        format!("{} Art.{}", self.doc_title, self.article_number)
    }

    fn description(&self) -> String {
        let body = format!(
            "Article {}: {}\n\n{}",
            self.article_number, self.article_title, self.content
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
        format!("{}:art{}", self.doc_id, self.article_number)
    }
}

#[derive(Debug, Clone)]
pub struct CanonDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<CanonItem>,
}

impl Document for CanonDoc {
    type Item = CanonItem;
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
