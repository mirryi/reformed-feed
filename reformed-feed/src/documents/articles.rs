use doc_feed::document::{Document, IntoFeedEntry};

use super::html_escape;

#[derive(Debug, Clone)]
pub struct ArticleItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: u32,
    pub name: String,
    pub text: String,
}

impl IntoFeedEntry for ArticleItem {
    fn title(&self) -> String {
        format!("{} Art.{}", self.doc_title, self.number)
    }
    fn description(&self) -> String {
        let body = format!("Article {}: {}\n\n{}", self.number, self.name, self.text);
        format!(
            "<div style='white-space: pre-wrap;'>{}</div>",
            html_escape(&body)
        )
    }
    fn guid(&self) -> String {
        format!("{}:art{}", self.doc_id, self.number)
    }
}

#[derive(Debug, Clone)]
pub struct ArticlesDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ArticleItem>,
}

impl Document for ArticlesDoc {
    type Item = ArticleItem;
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
