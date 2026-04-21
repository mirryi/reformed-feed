use doc_feed::document::{Document, IntoFeedEntry};

use super::{format_proofs, html_escape, Proof};

#[derive(Debug, Clone)]
pub struct CatechismItem {
    pub doc_id: String,
    pub doc_title: String,
    pub number: String,
    pub question: String,
    pub answer: String,
    pub answer_with_proofs: Option<String>,
    pub proofs: Vec<Proof>,
}

impl IntoFeedEntry for CatechismItem {
    fn title(&self) -> String {
        format!("{} Q{}", self.doc_title, self.number)
    }

    fn description(&self) -> String {
        let body = format!("Q{}. {}\n\nA. {}", self.number, self.question, self.answer);
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
        format!("{}:q{}", self.doc_id, self.number)
    }
}

#[derive(Debug, Clone)]
pub struct CatechismDoc {
    pub id: String,
    pub title: String,
    pub items: Vec<CatechismItem>,
}

impl Document for CatechismDoc {
    type Item = CatechismItem;
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
