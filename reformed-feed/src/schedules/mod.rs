pub mod frequency;
pub mod ordering;
pub mod presets;

use chrono::{DateTime, Utc};
use crate::feed::schedule::{Frequency, ItemOrder, ItemRef, Schedule};
use serde::{Deserialize, Serialize};

/// A schedule composed from an ItemOrder and a Frequency.
pub struct ComposedSchedule<O: ItemOrder, F: Frequency> {
    pub order: O,
    pub freq: F,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposedState<OS> {
    pub order_state: OS,
    pub last_emitted: Option<DateTime<Utc>>,
}

impl<O: ItemOrder, F: Frequency> Schedule for ComposedSchedule<O, F> {
    type State = ComposedState<O::State>;

    fn init_state(&self, doc_ids: &[&str], doc_lengths: &[usize]) -> Self::State {
        ComposedState {
            order_state: self.order.init_state(doc_ids, doc_lengths),
            last_emitted: None,
        }
    }

    fn next_if_due(&self, state: &Self::State, now: DateTime<Utc>) -> Option<ItemRef> {
        if !self.freq.should_emit(state.last_emitted, now) {
            return None;
        }
        self.order.next(&state.order_state)
    }

    fn advance(&self, state: &mut Self::State, emitted: &ItemRef, now: DateTime<Utc>) {
        self.order.advance(&mut state.order_state, emitted);
        state.last_emitted = Some(now);
    }
}
