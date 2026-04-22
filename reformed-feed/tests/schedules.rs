use chrono::{TimeZone, Utc};
use reformed_feed::feed::schedule::{Frequency, ItemOrder, Schedule};
use reformed_feed::schedules::frequency::{Daily, EveryNHours, NPerWeek};
use reformed_feed::schedules::ordering::{RoundRobin, Sequential, Weighted};
use reformed_feed::schedules::presets;
use std::collections::HashMap;

#[test]
fn sequential_finishes_doc0_before_doc1() {
    let order = Sequential;
    let mut state = order.init_state(&["a", "b"], &[3, 2]);
    for i in 0..3 {
        let item = order.next(&state).unwrap();
        assert_eq!(item.doc_id, "a");
        assert_eq!(item.item_index, i);
        order.advance(&mut state, &item);
    }
    for i in 0..2 {
        let item = order.next(&state).unwrap();
        assert_eq!(item.doc_id, "b");
        assert_eq!(item.item_index, i);
        order.advance(&mut state, &item);
    }
    let item = order.next(&state).unwrap();
    assert_eq!(item.doc_id, "a");
    assert_eq!(item.item_index, 0);
}

#[test]
fn round_robin_alternates() {
    let order = RoundRobin;
    let mut state = order.init_state(&["a", "b"], &[5, 5]);
    let item0 = order.next(&state).unwrap();
    assert_eq!(item0.doc_id, "a");
    order.advance(&mut state, &item0);
    let item1 = order.next(&state).unwrap();
    assert_eq!(item1.doc_id, "b");
    order.advance(&mut state, &item1);
    let item2 = order.next(&state).unwrap();
    assert_eq!(item2.doc_id, "a");
    assert_eq!(item2.item_index, 1);
}

#[test]
fn weighted_respects_ratios() {
    let mut weights = HashMap::new();
    weights.insert("a".to_string(), 3);
    weights.insert("b".to_string(), 1);
    let order = Weighted { weights };
    let mut state = order.init_state(&["a", "b"], &[100, 100]);
    let mut a_count = 0;
    let mut b_count = 0;
    for _ in 0..20 {
        let item = order.next(&state).unwrap();
        if item.doc_id == "a" { a_count += 1; } else { b_count += 1; }
        order.advance(&mut state, &item);
    }
    assert!(a_count >= 12, "a_count {} should be >= 12", a_count);
    assert!(b_count >= 3, "b_count {} should be >= 3", b_count);
}

#[test]
fn daily_gates_on_hour() {
    let freq = Daily { hour: 8 };
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 8, 0, 0).unwrap();
    assert!(freq.should_emit(None, now));
    let ten_later = Utc.with_ymd_and_hms(2026, 1, 1, 18, 0, 0).unwrap();
    assert!(!freq.should_emit(Some(now), ten_later));
    let next_day = Utc.with_ymd_and_hms(2026, 1, 2, 8, 0, 0).unwrap();
    assert!(freq.should_emit(Some(now), next_day));
}

#[test]
fn every_n_hours_respects_interval() {
    let freq = EveryNHours { interval_hours: 6 };
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    assert!(freq.should_emit(None, now));
    let three_later = Utc.with_ymd_and_hms(2026, 1, 1, 3, 0, 0).unwrap();
    assert!(!freq.should_emit(Some(now), three_later));
    let six_later = Utc.with_ymd_and_hms(2026, 1, 1, 6, 0, 0).unwrap();
    assert!(freq.should_emit(Some(now), six_later));
}

#[test]
fn n_per_week_spreads_across_week() {
    let freq = NPerWeek { items_per_week: 7 };
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    assert!(freq.should_emit(None, now));
    let twenty_hours = Utc.with_ymd_and_hms(2026, 1, 1, 20, 0, 0).unwrap();
    assert!(!freq.should_emit(Some(now), twenty_hours));
    let twenty_five = Utc.with_ymd_and_hms(2026, 1, 2, 1, 0, 0).unwrap();
    assert!(freq.should_emit(Some(now), twenty_five));
}

#[test]
fn composed_schedule_init_next_advance() {
    let schedule = presets::daily_sequential(8);
    let mut state = schedule.init_state(&["a", "b"], &[3, 2]);
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 8, 0, 0).unwrap();
    let item = schedule.next_if_due(&state, now).unwrap();
    assert_eq!(item.doc_id, "a");
    assert_eq!(item.item_index, 0);
    schedule.advance(&mut state, &item, now);
    assert!(state.last_emitted.is_some());
    let same_day = Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap();
    assert!(schedule.next_if_due(&state, same_day).is_none());
}

#[test]
fn all_presets_construct_and_work() {
    let doc_ids = &["a", "b"];
    let doc_lens = &[5_usize, 5];
    let now = Utc.with_ymd_and_hms(2026, 1, 1, 8, 0, 0).unwrap();

    let s = presets::daily_sequential(8);
    let state = s.init_state(doc_ids, doc_lens);
    assert!(s.next_if_due(&state, now).is_some());

    let s = presets::daily_round_robin(8);
    let state = s.init_state(doc_ids, doc_lens);
    assert!(s.next_if_due(&state, now).is_some());

    let mut weights = HashMap::new();
    weights.insert("a".to_string(), 2);
    weights.insert("b".to_string(), 1);
    let s = presets::weighted_daily(8, weights);
    let state = s.init_state(doc_ids, doc_lens);
    assert!(s.next_if_due(&state, now).is_some());

    let s = presets::frequent(6);
    let state = s.init_state(doc_ids, doc_lens);
    assert!(s.next_if_due(&state, now).is_some());

    let s = presets::weekly_digest(5);
    let state = s.init_state(doc_ids, doc_lens);
    assert!(s.next_if_due(&state, now).is_some());
}
