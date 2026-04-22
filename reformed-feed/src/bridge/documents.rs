use crate::feed::document::{Document, IntoFeedEntry};
use reformed_creeds::types::{format_proofs, html_escape};
use reformed_creeds::types::articles::{ArticleItem, ArticlesDoc};
use reformed_creeds::types::canon::{CanonItem, CanonDoc};
use reformed_creeds::types::catechism::{CatechismItem, CatechismDoc};
use reformed_creeds::types::confession::{ConfessionItem, ConfessionDoc};
use reformed_creeds::types::creed::{CreedItem, CreedDoc};
use reformed_creeds::types::theses::{ThesisItem, ThesesDoc};

// --- CatechismItem ---

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

// --- ConfessionItem ---

impl IntoFeedEntry for ConfessionItem {
    fn title(&self) -> String {
        format!(
            "{} Ch.{} Sec.{}",
            self.doc_title, self.chapter_number, self.section_number
        )
    }

    fn description(&self) -> String {
        let body = format!(
            "Chapter {}: {}\nSection {}\n\n{}",
            self.chapter_number, self.chapter_title, self.section_number, self.content
        );
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
        format!(
            "{}:ch{}:sec{}",
            self.doc_id, self.chapter_number, self.section_number
        )
    }
}

impl Document for ConfessionDoc {
    type Item = ConfessionItem;
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

// --- CanonItem ---

impl IntoFeedEntry for CanonItem {
    fn title(&self) -> String {
        format!("{} Art.{}", self.doc_title, self.article_number)
    }

    fn description(&self) -> String {
        let body = format!(
            "Article {}: {}\n\n{}",
            self.article_number, self.article_title, self.content
        );
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
        format!("{}:art{}", self.doc_id, self.article_number)
    }
}

impl Document for CanonDoc {
    type Item = CanonItem;
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

// --- CreedItem ---

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

// --- ArticleItem ---

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

// --- ThesisItem ---

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
