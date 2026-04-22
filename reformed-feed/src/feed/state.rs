use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};

/// A feed entry serialized for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredFeedEntry {
    pub title: String,
    pub description: String,
    pub guid: String,
    pub pub_date: DateTime<Utc>,
}

/// Top-level persisted state, generic over schedule state.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(
    serialize = "S: Serialize",
    deserialize = "S: DeserializeOwned"
))]
pub struct PersistedState<S> {
    pub schedule_state: S,
    pub recent_items: VecDeque<StoredFeedEntry>,
}

/// Persistence layer for state.
pub trait StateStore<S: Serialize + DeserializeOwned> {
    fn load(&self) -> Result<Option<PersistedState<S>>>;
    fn save(&self, state: &PersistedState<S>) -> Result<()>;
}

/// JSON file-based state store.
pub struct JsonFileStore {
    path: PathBuf,
}

impl JsonFileStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl<S: Serialize + DeserializeOwned> StateStore<S> for JsonFileStore {
    fn load(&self) -> Result<Option<PersistedState<S>>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(&self.path)?;
        let state = serde_json::from_str(&content)?;
        Ok(Some(state))
    }

    fn save(&self, state: &PersistedState<S>) -> Result<()> {
        let content = serde_json::to_string_pretty(state)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
