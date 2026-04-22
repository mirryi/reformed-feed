use reformed_creeds::parse;
use reformed_feed::feed::document::{Document, IntoFeedEntry};
use std::collections::HashSet;

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn read_creeds_json(filename: &str) -> Vec<u8> {
    std::fs::read(workspace_root().join(format!("data/Creeds.json/creeds/{}", filename))).unwrap()
}

fn read_compendium(filename: &str) -> Vec<u8> {
    std::fs::read(workspace_root().join(format!("data/compendium/data/{}", filename))).unwrap()
}

fn assert_feed_entries_valid<D: Document>(doc: &D) {
    let items = doc.items();
    assert!(!items.is_empty(), "document should have items");

    let mut guids = HashSet::new();
    for item in &items {
        let title = item.title();
        let desc = item.description();
        let guid = item.guid();
        assert!(!title.is_empty(), "title should not be empty");
        assert!(!desc.is_empty(), "description should not be empty");
        assert!(!guid.is_empty(), "guid should not be empty");
        assert!(guids.insert(guid.clone()), "duplicate guid: {}", guid);
    }
}

#[test]
fn catechism_feed_entries() {
    let data = read_creeds_json("westminster_shorter_catechism.json");
    let doc = parse::catechism::parse_by_question(
        "westminster-shorter-catechism",
        "Westminster Shorter Catechism",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
    let first = &doc.items[0];
    assert!(first.title().contains("Q1"));
    assert!(first.description().contains("glorify God"));
}

#[test]
fn confession_feed_entries() {
    let data = read_creeds_json("westminster_confession_of_faith.json");
    let doc = parse::confession::parse_by_section(
        "westminster-confession",
        "Westminster Confession of Faith",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
    let first = &doc.items[0];
    assert!(first.title().contains("Ch.1"));
    assert!(first.title().contains("Sec.1"));
}

#[test]
fn canon_feed_entries() {
    let data = read_creeds_json("belgic_confession_of_faith.json");
    let doc = parse::canon::parse_by_article(
        "belgic-confession",
        "Belgic Confession",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
    let first = &doc.items[0];
    assert!(first.title().contains("Art.1"));
}

#[test]
fn creed_feed_entries() {
    let data = read_creeds_json("apostles_creed.json");
    let doc = parse::creed::parse_whole(
        "apostles-creed",
        "Apostles' Creed",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
}

#[test]
fn articles_feed_entries() {
    let data = read_compendium("anglican/39-articles.yaml");
    let doc = parse::articles::parse_by_article(
        "39-articles",
        "Thirty-nine Articles of Religion",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
    let first = &doc.items[0];
    assert!(first.title().contains("Art.1"));
}

#[test]
fn theses_feed_entries() {
    let data = read_compendium("reformation/95-theses.yaml");
    let doc = parse::theses::parse_by_thesis(
        "95-theses",
        "Martin Luther's 95 Theses",
        &data,
    )
    .unwrap();
    assert_feed_entries_valid(&doc);
    let first = &doc.items[0];
    assert!(first.title().contains("Thesis 1"));
}
