use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub feed: FeedConfig,
    #[serde(default)]
    pub sources: SourcesConfig,
    pub schedule: ScheduleConfig,
    #[serde(default)]
    pub documents: Vec<DocumentEntry>,
}

#[derive(Debug, Deserialize)]
pub struct FeedConfig {
    pub title: String,
    pub description: String,
    pub link: String,
    #[serde(default = "default_max_items")]
    pub max_items: usize,
}

fn default_max_items() -> usize {
    10
}

#[derive(Debug, Deserialize)]
pub struct SourcesConfig {
    #[serde(default = "default_creeds_json")]
    pub creeds_json: String,
    #[serde(default = "default_compendium")]
    pub compendium: String,
}

fn default_creeds_json() -> String {
    "data/Creeds.json".to_string()
}

fn default_compendium() -> String {
    "data/compendium".to_string()
}

impl Default for SourcesConfig {
    fn default() -> Self {
        Self {
            creeds_json: default_creeds_json(),
            compendium: default_compendium(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScheduleConfig {
    pub preset: String,
    pub hour: Option<u8>,
    pub interval_hours: Option<u32>,
    pub items_per_week: Option<u32>,
    pub weights: Option<HashMap<String, u32>>,
}

#[derive(Debug, Deserialize)]
pub struct DocumentEntry {
    pub id: String,
    pub parse: Option<String>,
    pub file: Option<String>,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
