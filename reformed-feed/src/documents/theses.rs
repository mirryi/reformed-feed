use doc_feed::document::{Document, IntoFeedEntry};

use super::html_escape;

#[derive(Debug, Clone)]
pub struct ThesisItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: u32,
    pub text: String,
}

impl IntoFeedEntry for ThesisItem {
    fn title(&self) -> String {
        format!("{} Thesis {}", self.doc_title, self.number)
    }
    fn description(&self) -> String {
        let body = format!("Thesis {}\n\n{}", self.number, self.text);
        format!(
            "<div style='white-space: pre-wrap;'>{}</div>",
            html_escape(&body)
        )
    }
    fn guid(&self) -> String {
        format!("{}:thesis{}", self.doc_id, self.number)
    }
}

#[derive(Debug, Clone)]
pub struct ThesesDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ThesisItem>,
}

impl Document for ThesesDoc {
    type Item = ThesisItem;
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
