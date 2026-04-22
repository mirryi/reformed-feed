use super::frequency::{Daily, EveryNHours, NPerWeek};
use super::ordering::{Proportional, RoundRobin, Sequential, Weighted};
use super::ComposedSchedule;
use std::collections::HashMap;

pub type DailySequential = ComposedSchedule<Sequential, Daily>;
pub type DailyRoundRobin = ComposedSchedule<RoundRobin, Daily>;
pub type WeightedDaily = ComposedSchedule<Weighted, Daily>;
pub type DailyProportional = ComposedSchedule<Proportional, Daily>;
pub type Frequent = ComposedSchedule<RoundRobin, EveryNHours>;
pub type WeeklyDigest = ComposedSchedule<Sequential, NPerWeek>;

pub fn daily_sequential(hour: u8) -> DailySequential {
    ComposedSchedule {
        order: Sequential,
        freq: Daily { hour },
    }
}

pub fn daily_round_robin(hour: u8) -> DailyRoundRobin {
    ComposedSchedule {
        order: RoundRobin,
        freq: Daily { hour },
    }
}

pub fn weighted_daily(hour: u8, weights: HashMap<String, u32>) -> WeightedDaily {
    ComposedSchedule {
        order: Weighted { weights },
        freq: Daily { hour },
    }
}

pub fn daily_proportional(hour: u8) -> DailyProportional {
    ComposedSchedule {
        order: Proportional,
        freq: Daily { hour },
    }
}

pub fn frequent(interval_hours: u32) -> Frequent {
    ComposedSchedule {
        order: RoundRobin,
        freq: EveryNHours { interval_hours },
    }
}

pub fn weekly_digest(items_per_week: u32) -> WeeklyDigest {
    ComposedSchedule {
        order: Sequential,
        freq: NPerWeek { items_per_week },
    }
}
