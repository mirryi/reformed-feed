use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreedItem {
    pub doc_id: String,
    pub doc_title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreedDoc {
    pub id: String,
    pub title: String,
    pub content: String,
}
