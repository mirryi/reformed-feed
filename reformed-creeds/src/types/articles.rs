use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: u32,
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ArticleItem>,
}
