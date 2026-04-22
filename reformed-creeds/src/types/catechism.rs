use serde::{Deserialize, Serialize};

use super::Proof;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatechismItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: String,
    pub question: String,
    pub answer: String,
    pub answer_with_proofs: Option<String>,
    pub proofs: Vec<Proof>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatechismDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<CatechismItem>,
}
