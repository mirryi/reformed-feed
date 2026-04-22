use anyhow::Result;
use crate::feed::document::ParseStrategy;
use reformed_creeds::types::articles::ArticlesDoc;
use reformed_creeds::types::canon::CanonDoc;
use reformed_creeds::types::catechism::CatechismDoc;
use reformed_creeds::types::confession::ConfessionDoc;
use reformed_creeds::types::creed::CreedDoc;
use reformed_creeds::types::theses::ThesesDoc;

pub struct ByQuestion {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByQuestion {
    type Doc = CatechismDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::catechism::parse_by_question(&self.doc_id, &self.doc_title, data)
    }
}

pub struct BySection {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for BySection {
    type Doc = ConfessionDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::confession::parse_by_section(&self.doc_id, &self.doc_title, data)
    }
}

pub struct ByChapter {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByChapter {
    type Doc = ConfessionDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::confession::parse_by_chapter(&self.doc_id, &self.doc_title, data)
    }
}

pub struct CanonByArticle {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for CanonByArticle {
    type Doc = CanonDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::canon::parse_by_article(&self.doc_id, &self.doc_title, data)
    }
}

pub struct WholeCreed {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for WholeCreed {
    type Doc = CreedDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::creed::parse_whole(&self.doc_id, &self.doc_title, data)
    }
}

pub struct ArticlesByArticle {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ArticlesByArticle {
    type Doc = ArticlesDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::articles::parse_by_article(&self.doc_id, &self.doc_title, data)
    }
}

pub struct ByThesis {
    pub doc_id: String,
    pub doc_title: String,
}

impl ParseStrategy for ByThesis {
    type Doc = ThesesDoc;
    fn parse(&self, data: &[u8]) -> Result<Self::Doc> {
        reformed_creeds::parse::theses::parse_by_thesis(&self.doc_id, &self.doc_title, data)
    }
}
