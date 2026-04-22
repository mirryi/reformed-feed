use crate::feed::schedule::{ItemOrder, ItemRef};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sequential: finish all items in one document before moving to the next.
pub struct Sequential;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequentialState {
    pub doc_ids: Vec<String>,
    pub doc_lengths: Vec<usize>,
    pub current_doc: usize,
    pub current_item: usize,
}

impl ItemOrder for Sequential {
    type State = SequentialState;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State {
        SequentialState {
            doc_ids: doc_ids.iter().map(|s| s.to_string()).collect(),
            doc_lengths: doc_lengths.to_vec(),
            current_doc: 0,
            current_item: 0,
        }
    }

    fn next(&self, state: &Self::State) -> Option<ItemRef> {
        if state.doc_ids.is_empty() {
            return None;
        }
        let num_docs = state.doc_ids.len();
        let mut doc = state.current_doc % num_docs;
        let mut item = state.current_item;

        for _ in 0..num_docs {
            if item < state.doc_lengths[doc] {
                return Some(ItemRef {
                    doc_id: state.doc_ids[doc].clone(),
                    item_index: item,
                });
            }
            doc = (doc + 1) % num_docs;
            item = 0;
        }
        // All exhausted — wrap around (new cycle)
        Some(ItemRef {
            doc_id: state.doc_ids[0].clone(),
            item_index: 0,
        })
    }

    fn advance(&self, state: &mut Self::State, _emitted: &ItemRef) {
        let num_docs = state.doc_ids.len();
        if num_docs == 0 {
            return;
        }
        state.current_item += 1;
        let doc_len = state.doc_lengths[state.current_doc % num_docs];
        if state.current_item >= doc_len {
            state.current_item = 0;
            state.current_doc += 1;
            if state.current_doc >= num_docs {
                state.current_doc = 0;
            }
        }
    }
}

/// RoundRobin: one item from each document in turn.
pub struct RoundRobin;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoundRobinState {
    pub doc_ids: Vec<String>,
    pub doc_lengths: Vec<usize>,
    pub positions: HashMap<String, usize>,
    pub turn: usize,
}

impl ItemOrder for RoundRobin {
    type State = RoundRobinState;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State {
        let positions = doc_ids.iter().map(|id| (id.to_string(), 0)).collect();
        RoundRobinState {
            doc_ids: doc_ids.iter().map(|s| s.to_string()).collect(),
            doc_lengths: doc_lengths.to_vec(),
            positions,
            turn: 0,
        }
    }

    fn next(&self, state: &Self::State) -> Option<ItemRef> {
        if state.doc_ids.is_empty() {
            return None;
        }
        let num_docs = state.doc_ids.len();
        let doc_idx = state.turn % num_docs;
        let doc_id = &state.doc_ids[doc_idx];
        let pos = *state.positions.get(doc_id).unwrap_or(&0);
        let len = state.doc_lengths[doc_idx];
        let item_index = pos % len;
        Some(ItemRef {
            doc_id: doc_id.clone(),
            item_index,
        })
    }

    fn advance(&self, state: &mut Self::State, emitted: &ItemRef) {
        if let Some(pos) = state.positions.get_mut(&emitted.doc_id) {
            *pos += 1;
        }
        state.turn += 1;
    }
}

/// Proportional: documents get turns proportional to their length,
/// so all documents finish their first pass at roughly the same time.
pub struct Proportional;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProportionalState {
    pub doc_ids: Vec<String>,
    pub doc_lengths: Vec<usize>,
    pub positions: HashMap<String, usize>,
    pub accumulated: Vec<f64>,
    pub turn: usize,
}

impl ItemOrder for Proportional {
    type State = ProportionalState;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State {
        let positions = doc_ids.iter().map(|id| (id.to_string(), 0)).collect();
        let accumulated = vec![0.0; doc_ids.len()];
        ProportionalState {
            doc_ids: doc_ids.iter().map(|s| s.to_string()).collect(),
            doc_lengths: doc_lengths.to_vec(),
            positions,
            accumulated,
            turn: 0,
        }
    }

    fn next(&self, state: &Self::State) -> Option<ItemRef> {
        if state.doc_ids.is_empty() {
            return None;
        }
        let total_length: usize = state.doc_lengths.iter().sum();
        if total_length == 0 {
            return None;
        }

        let mut best_idx = 0;
        let mut best_deficit = f64::NEG_INFINITY;
        for (i, &len) in state.doc_lengths.iter().enumerate() {
            let expected = (len as f64 / total_length as f64) * (state.turn as f64 + 1.0);
            let actual = state.accumulated[i];
            let deficit = expected - actual;
            if deficit > best_deficit {
                best_deficit = deficit;
                best_idx = i;
            }
        }

        let doc_id = &state.doc_ids[best_idx];
        let pos = *state.positions.get(doc_id).unwrap_or(&0);
        let len = state.doc_lengths[best_idx];
        let item_index = pos % len;
        Some(ItemRef {
            doc_id: doc_id.clone(),
            item_index,
        })
    }

    fn advance(&self, state: &mut Self::State, emitted: &ItemRef) {
        if let Some(idx) = state.doc_ids.iter().position(|id| id == &emitted.doc_id) {
            state.accumulated[idx] += 1.0;
        }
        if let Some(pos) = state.positions.get_mut(&emitted.doc_id) {
            *pos += 1;
        }
        state.turn += 1;
    }
}

/// Weighted: documents get turns proportional to configured weights.
pub struct Weighted {
    pub weights: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedState {
    pub doc_ids: Vec<String>,
    pub doc_lengths: Vec<usize>,
    pub weights: Vec<u32>,
    pub positions: HashMap<String, usize>,
    pub accumulated: Vec<f64>,
    pub turn: usize,
}

impl ItemOrder for Weighted {
    type State = WeightedState;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State {
        let weights: Vec<u32> = doc_ids
            .iter()
            .map(|id| *self.weights.get(*id).unwrap_or(&1))
            .collect();
        let positions = doc_ids.iter().map(|id| (id.to_string(), 0)).collect();
        let accumulated = vec![0.0; doc_ids.len()];
        WeightedState {
            doc_ids: doc_ids.iter().map(|s| s.to_string()).collect(),
            doc_lengths: doc_lengths.to_vec(),
            weights,
            positions,
            accumulated,
            turn: 0,
        }
    }

    fn next(&self, state: &Self::State) -> Option<ItemRef> {
        if state.doc_ids.is_empty() {
            return None;
        }
        let total_weight: u32 = state.weights.iter().sum();
        if total_weight == 0 {
            return None;
        }

        let mut best_idx = 0;
        let mut best_deficit = f64::NEG_INFINITY;
        for (i, &w) in state.weights.iter().enumerate() {
            let expected = (w as f64 / total_weight as f64) * (state.turn as f64 + 1.0);
            let actual = state.accumulated[i];
            let deficit = expected - actual;
            if deficit > best_deficit {
                best_deficit = deficit;
                best_idx = i;
            }
        }

        let doc_id = &state.doc_ids[best_idx];
        let pos = *state.positions.get(doc_id).unwrap_or(&0);
        let len = state.doc_lengths[best_idx];
        let item_index = pos % len;
        Some(ItemRef {
            doc_id: doc_id.clone(),
            item_index,
        })
    }

    fn advance(&self, state: &mut Self::State, emitted: &ItemRef) {
        if let Some(idx) = state.doc_ids.iter().position(|id| id == &emitted.doc_id) {
            state.accumulated[idx] += 1.0;
        }
        if let Some(pos) = state.positions.get_mut(&emitted.doc_id) {
            *pos += 1;
        }
        state.turn += 1;
    }
}
