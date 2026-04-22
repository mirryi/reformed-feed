use serde::{Deserialize, Serialize};

use super::Proof;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonItem {
    pub doc_id: String,
    pub doc_title: String,
    pub article_number: String,
    pub article_title: String,
    pub content: String,
    pub content_with_proofs: Option<String>,
    pub proofs: Vec<Proof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<CanonItem>,
}
