use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesisItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: u32,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThesesDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ThesisItem>,
}
