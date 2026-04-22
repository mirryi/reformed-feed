use serde::{Deserialize, Serialize};

use super::Proof;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfessionDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<ConfessionItem>,
}
