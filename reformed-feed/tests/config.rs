#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Config {
    feed: FeedConfig,
    #[serde(default)]
    sources: SourcesConfig,
    schedule: ScheduleConfig,
    #[serde(default)]
    documents: Vec<DocumentEntry>,
}

#[derive(Debug, Deserialize)]
struct FeedConfig {
    title: String,
    description: String,
    link: String,
    #[serde(default = "default_max_items")]
    max_items: usize,
}

fn default_max_items() -> usize { 10 }

#[derive(Debug, Deserialize)]
struct SourcesConfig {
    #[serde(default = "default_creeds")]
    creeds_json: String,
    #[serde(default = "default_compendium")]
    compendium: String,
}

fn default_creeds() -> String { "data/Creeds.json".to_string() }
fn default_compendium() -> String { "data/compendium".to_string() }

impl Default for SourcesConfig {
    fn default() -> Self {
        Self { creeds_json: default_creeds(), compendium: default_compendium() }
    }
}

#[derive(Debug, Deserialize)]
struct ScheduleConfig {
    preset: String,
    hour: Option<u8>,
    interval_hours: Option<u32>,
    items_per_week: Option<u32>,
    weights: Option<HashMap<String, u32>>,
}

#[derive(Debug, Deserialize)]
struct DocumentEntry {
    id: String,
    parse: Option<String>,
    file: Option<String>,
}

#[test]
fn parse_example_config() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let content = std::fs::read_to_string(root.join("etc/config.toml")).unwrap();
    let config: Config = toml::from_str(&content).unwrap();
    assert_eq!(config.feed.title, "Reformed Daily");
    assert_eq!(config.feed.max_items, 10);
    assert_eq!(config.schedule.preset, "daily-round-robin");
    assert_eq!(config.schedule.hour, Some(4));
    assert_eq!(config.documents.len(), 22);
}

#[test]
fn config_defaults() {
    let minimal = r#"
[feed]
title = "Test"
description = "Test feed"
link = "https://example.com"

[schedule]
preset = "daily-sequential"
"#;
    let config: Config = toml::from_str(minimal).unwrap();
    assert_eq!(config.feed.max_items, 10);
    assert_eq!(config.sources.creeds_json, "data/Creeds.json");
    assert_eq!(config.sources.compendium, "data/compendium");
    assert!(config.documents.is_empty());
}
