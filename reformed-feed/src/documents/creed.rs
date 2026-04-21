use doc_feed::document::{Document, IntoFeedEntry};

use super::html_escape;

#[derive(Debug, Clone)]
pub struct CreedItem {
    pub doc_id: String,
    pub doc_title: String,
    pub content: String,
}

impl IntoFeedEntry for CreedItem {
    fn title(&self) -> String {
        self.doc_title.clone()
    }
    fn description(&self) -> String {
        format!(
            "<div style='white-space: pre-wrap;'>{}</div>",
            html_escape(&self.content)
        )
    }
    fn guid(&self) -> String {
        format!("{}:full", self.doc_id)
    }
}

#[derive(Debug, Clone)]
pub struct CreedDoc {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl Document for CreedDoc {
    type Item = CreedItem;
    fn items(&self) -> Vec<Self::Item> {
        vec![CreedItem {
            doc_id: self.id.clone(),
            doc_title: self.title.clone(),
            content: self.content.clone(),
        }]
    }
    fn id(&self) -> &str {
        &self.id
    }
    fn title(&self) -> &str {
        &self.title
    }
}
