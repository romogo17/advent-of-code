// Millimeters per millisecond
const VELOCITY: u32 = 1;

/// The distance `y` is a quadratic function `y = x(t - x)`,
/// where `x` is the hold time and `t` is the max time
fn distance(hold_time_ms: u32, max_time_ms: u32) -> u32 {
    VELOCITY * hold_time_ms * (max_time_ms - hold_time_ms)
}

#[derive(Debug)]
pub struct BoatRace {
    max_time: u32,
    record_distance: u32,
}

impl BoatRace {
    pub fn new(max_time: u32, record_distance: u32) -> Self {
        Self {
            max_time,
            record_distance,
        }
    }

    /// The naive way to get the number of ways to win is to calculate
    /// all the distances for all the hold times and compare them to the
    /// record distance.
    pub fn naive_ways_to_win(&self) -> u32 {
        (1..self.max_time)
            .filter_map(|hold_time| {
                use std::cmp::*;

                match distance(hold_time, self.max_time).cmp(&self.record_distance) {
                    Ordering::Greater => Some(hold_time),
                    _ => None,
                }
            })
            .count() as u32
    }
}
