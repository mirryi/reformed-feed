use reformed_creeds::parse;

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

#[test]
fn parse_catechism_by_question() {
    let data = read_creeds_json("westminster_shorter_catechism.json");
    let doc = parse::catechism::parse_by_question(
        "westminster-shorter-catechism",
        "Westminster Shorter Catechism",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 107);
    assert_eq!(doc.items[0].number, "1");
    assert_eq!(doc.items[0].question, "What is the chief end of man?");
    assert!(doc.items[0].answer.contains("glorify God"));
}

#[test]
fn parse_confession_by_section() {
    let data = read_creeds_json("westminster_confession_of_faith.json");
    let doc = parse::confession::parse_by_section(
        "westminster-confession",
        "Westminster Confession of Faith",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 172);
    assert_eq!(doc.items[0].chapter_number, "1");
    assert_eq!(doc.items[0].section_number, "1");
    assert!(doc.items[0].content.contains("light of nature"));
}

#[test]
fn parse_confession_by_chapter() {
    let data = read_creeds_json("westminster_confession_of_faith.json");
    let doc = parse::confession::parse_by_chapter(
        "westminster-confession",
        "Westminster Confession of Faith",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 33);
    assert_eq!(doc.items[0].chapter_number, "1");
    assert_eq!(doc.items[0].section_number, "all");
}

#[test]
fn parse_canon_by_article() {
    let data = read_creeds_json("belgic_confession_of_faith.json");
    let doc = parse::canon::parse_by_article(
        "belgic-confession",
        "Belgic Confession",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 37);
    assert_eq!(doc.items[0].article_number, "1");
    assert_eq!(doc.items[0].article_title, "The Only God");
}

#[test]
fn parse_creed_whole() {
    let data = read_creeds_json("apostles_creed.json");
    let doc = parse::creed::parse_whole(
        "apostles-creed",
        "Apostles' Creed",
        &data,
    )
    .unwrap();
    assert!(!doc.content.is_empty());
    assert!(doc.content.contains("I believe"));
}

#[test]
fn parse_articles_by_article() {
    let data = read_compendium("anglican/39-articles.yaml");
    let doc = parse::articles::parse_by_article(
        "39-articles",
        "Thirty-nine Articles of Religion",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 39);
    assert_eq!(doc.items[0].number, 1);
    assert!(doc.items[0].name.contains("Holy Trinity"));
}

#[test]
fn parse_theses_by_thesis() {
    let data = read_compendium("reformation/95-theses.yaml");
    let doc = parse::theses::parse_by_thesis(
        "95-theses",
        "Martin Luther's 95 Theses",
        &data,
    )
    .unwrap();
    assert_eq!(doc.items.len(), 95);
    assert_eq!(doc.items[0].number, 1);
    assert!(doc.items[0].text.contains("Repent"));
}
