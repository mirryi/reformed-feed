use chrono::{DateTime, Timelike, Utc};
use doc_feed::schedule::Frequency;

/// Emit once per day.
pub struct Daily {
    pub hour: u8,
}

impl Frequency for Daily {
    fn should_emit(&self, last_emitted: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool {
        match last_emitted {
            None => true,
            Some(last) => {
                // Emit if current hour is at or past the target hour
                // and at least 20 hours have passed (prevent double-emit)
                let hours_since = (now - last).num_hours();
                hours_since >= 20 && now.hour() >= self.hour as u32
            }
        }
    }
}

/// Emit every N hours.
pub struct EveryNHours {
    pub interval_hours: u32,
}

impl Frequency for EveryNHours {
    fn should_emit(&self, last_emitted: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool {
        match last_emitted {
            None => true,
            Some(last) => {
                let elapsed = now - last;
                elapsed >= chrono::Duration::hours(self.interval_hours as i64)
            }
        }
    }
}

/// Emit N items spread across the week.
pub struct NPerWeek {
    pub items_per_week: u32,
}

impl Frequency for NPerWeek {
    fn should_emit(&self, last_emitted: Option<DateTime<Utc>>, now: DateTime<Utc>) -> bool {
        match last_emitted {
            None => true,
            Some(last) => {
                if self.items_per_week == 0 {
                    return false;
                }
                let interval_hours = 168.0 / self.items_per_week as f64;
                let elapsed = (now - last).num_minutes() as f64 / 60.0;
                elapsed >= interval_hours
            }
        }
    }
}
