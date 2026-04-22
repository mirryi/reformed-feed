use reformed_creeds::registry::{lookup, REGISTRY};

#[test]
fn all_22_ids_resolve() {
    assert_eq!(REGISTRY.len(), 22);
    for entry in REGISTRY {
        let found = lookup(entry.id);
        assert!(found.is_some(), "lookup failed for: {}", entry.id);
        let found = found.unwrap();
        assert_eq!(found.id, entry.id);
    }
}

#[test]
fn unknown_id_returns_none() {
    assert!(lookup("nonexistent-document").is_none());
}

#[test]
fn entries_have_valid_fields() {
    for entry in REGISTRY {
        assert!(!entry.id.is_empty());
        assert!(!entry.title.is_empty(), "title empty for {}", entry.id);
        assert!(!entry.filename.is_empty(), "filename empty for {}", entry.id);
        assert!(!entry.default_parse.is_empty(), "default_parse empty for {}", entry.id);
    }
}
