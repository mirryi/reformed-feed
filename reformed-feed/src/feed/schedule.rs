use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};

/// Identifies a specific item within a specific document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, serde::Deserialize)]
pub struct ItemRef {
    pub doc_id: String,
    pub item_index: usize,
}

/// Determines which document/item comes next.
pub trait ItemOrder {
    type State: Serialize + DeserializeOwned + Clone;

    /// Initialize ordering state from document info.
    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State;
    /// Return the next item to emit, or None if all documents are exhausted.
    fn next(&self, state: &Self::State) -> Option<ItemRef>;
    /// Advance state after emitting an item.
    fn advance(&self, state: &mut Self::State, emitted: &ItemRef);
}

/// Determines when to emit the next item.
pub trait Frequency {
    /// Returns true if enough time has passed to emit a new item.
    fn should_emit(&self, last_emitted: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool;
}

/// User-facing schedule composing ordering and timing.
pub trait Schedule {
    type State: Serialize + DeserializeOwned + Clone;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State;
    /// Return the next item if one is due now, or None.
    fn next_if_due(&self, state: &Self::State, now: DateTime<Utc>) -> Option<ItemRef>;
    /// Advance state after emitting an item.
    fn advance(&self, state: &mut Self::State, emitted: &ItemRef, now: DateTime<Utc>);
}
