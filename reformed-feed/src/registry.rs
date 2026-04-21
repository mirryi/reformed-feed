/// Source type for a document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    CreedsJson,
    Compendium,
}

/// Document shape determines which document type and default parse strategy to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Catechism,
    Confession,
    Canon,
    Creed,
    Articles,
    Theses,
}

/// Registry entry for a known document.
#[derive(Debug, Clone)]
pub struct RegistryEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub source: Source,
    pub filename: &'static str,
    pub shape: Shape,
    pub default_parse: &'static str,
}

pub const REGISTRY: &[RegistryEntry] = &[
    RegistryEntry { id: "1695-baptist-catechism", title: "1695 Baptist Catechism", source: Source::CreedsJson, filename: "1695_baptist_catechism.json", shape: Shape::Catechism, default_parse: "by-question" },
    RegistryEntry { id: "apostles-creed", title: "Apostles' Creed", source: Source::CreedsJson, filename: "apostles_creed.json", shape: Shape::Creed, default_parse: "whole-document" },
    RegistryEntry { id: "athanasian-creed", title: "Athanasian Creed", source: Source::CreedsJson, filename: "athanasian_creed.json", shape: Shape::Creed, default_parse: "whole-document" },
    RegistryEntry { id: "belgic-confession", title: "Belgic Confession", source: Source::CreedsJson, filename: "belgic_confession_of_faith.json", shape: Shape::Canon, default_parse: "by-article" },
    RegistryEntry { id: "canons-of-dort", title: "Canons of Dort", source: Source::CreedsJson, filename: "canons_of_dort.json", shape: Shape::Confession, default_parse: "by-section" },
    RegistryEntry { id: "chalcedonian-definition", title: "Chalcedonian Definition", source: Source::CreedsJson, filename: "chalcedonian_definition.json", shape: Shape::Creed, default_parse: "whole-document" },
    RegistryEntry { id: "chicago-statement", title: "Chicago Statement on Biblical Inerrancy", source: Source::CreedsJson, filename: "chicago_statement_on_biblical_inerrancy.json", shape: Shape::Canon, default_parse: "by-article" },
    RegistryEntry { id: "first-confession-of-basel", title: "First Confession of Basel", source: Source::CreedsJson, filename: "first_confession_of_basel.json", shape: Shape::Canon, default_parse: "by-article" },
    RegistryEntry { id: "heidelberg-catechism", title: "Heidelberg Catechism", source: Source::CreedsJson, filename: "heidelberg_catechism.json", shape: Shape::Catechism, default_parse: "by-question" },
    RegistryEntry { id: "irenaeus-rule-of-faith", title: "Irenaeus' Rule of Faith", source: Source::CreedsJson, filename: "irenaeus_rule_of_faith.json", shape: Shape::Creed, default_parse: "whole-document" },
    RegistryEntry { id: "london-baptist-1689", title: "London Baptist Confession of 1689", source: Source::CreedsJson, filename: "london_baptist_1689.json", shape: Shape::Confession, default_parse: "by-section" },
    RegistryEntry { id: "nicene-creed", title: "Nicene Creed", source: Source::CreedsJson, filename: "nicene_creed.json", shape: Shape::Creed, default_parse: "whole-document" },
    RegistryEntry { id: "puritan-catechism", title: "Puritan Catechism", source: Source::CreedsJson, filename: "puritan_catechism.json", shape: Shape::Catechism, default_parse: "by-question" },
    RegistryEntry { id: "savoy-declaration", title: "Savoy Declaration", source: Source::CreedsJson, filename: "savoy_declaration.json", shape: Shape::Confession, default_parse: "by-section" },
    RegistryEntry { id: "scots-confession", title: "Scots Confession", source: Source::CreedsJson, filename: "scots_confession.json", shape: Shape::Canon, default_parse: "by-article" },
    RegistryEntry { id: "second-helvetic-confession", title: "Second Helvetic Confession", source: Source::CreedsJson, filename: "second_helvetic_confession.json", shape: Shape::Confession, default_parse: "by-section" },
    RegistryEntry { id: "waldensian-confession", title: "Waldensian Confession", source: Source::CreedsJson, filename: "waldensian_confession.json", shape: Shape::Canon, default_parse: "by-article" },
    RegistryEntry { id: "westminster-confession", title: "Westminster Confession of Faith", source: Source::CreedsJson, filename: "westminster_confession_of_faith.json", shape: Shape::Confession, default_parse: "by-section" },
    RegistryEntry { id: "westminster-larger-catechism", title: "Westminster Larger Catechism", source: Source::CreedsJson, filename: "westminster_larger_catechism.json", shape: Shape::Catechism, default_parse: "by-question" },
    RegistryEntry { id: "westminster-shorter-catechism", title: "Westminster Shorter Catechism", source: Source::CreedsJson, filename: "westminster_shorter_catechism.json", shape: Shape::Catechism, default_parse: "by-question" },
    RegistryEntry { id: "39-articles", title: "Thirty-nine Articles of Religion", source: Source::Compendium, filename: "anglican/39-articles.yaml", shape: Shape::Articles, default_parse: "by-article" },
    RegistryEntry { id: "95-theses", title: "Martin Luther's 95 Theses", source: Source::Compendium, filename: "reformation/95-theses.yaml", shape: Shape::Theses, default_parse: "by-thesis" },
];

/// Look up a registry entry by document ID.
pub fn lookup(id: &str) -> Option<&'static RegistryEntry> {
    REGISTRY.iter().find(|entry| entry.id == id)
}
