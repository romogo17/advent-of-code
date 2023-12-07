// Millimeters per millisecond
const VELOCITY: u64 = 1;

/// The distance `y` is a quadratic function `y = x(t - x)`,
/// where `x` is the hold time and `t` is the max time
fn distance(hold_time_ms: u64, max_time_ms: u64) -> u64 {
    VELOCITY * hold_time_ms * (max_time_ms - hold_time_ms)
}

/// Solve a quadratic equation `ax^2 + bx + c = 0`
fn solve_quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    let sqrt_discriminant = discriminant.sqrt();

    let x1 = (-b - sqrt_discriminant) / (2.0 * a);
    let x2 = (-b + sqrt_discriminant) / (2.0 * a);

    (x1, x2)
}

#[derive(Debug)]
pub struct BoatRace {
    max_time: u64,
    record_distance: u64,
}

impl BoatRace {
    pub fn new(max_time: u64, record_distance: u64) -> Self {
        Self {
            max_time,
            record_distance,
        }
    }

    /// The naive way to get the number of ways to win is to calculate
    /// all the distances for all the hold times and compare them to the
    /// record distance.
    pub fn naive_ways_to_win(&self) -> u64 {
        (1..self.max_time)
            .filter_map(|hold_time| {
                use std::cmp::*;

                match distance(hold_time, self.max_time).cmp(&self.record_distance) {
                    Ordering::Greater => Some(hold_time),
                    _ => None,
                }
            })
            .count() as u64
    }

    /// The optimal way to calculate the ways to win is to solve the quadratic
    /// equation `y = x(t - x)`, where:
    /// - `y` is the record distance,
    /// - `x` is the hold time, and
    /// - `t` is the max time.
    ///
    /// In standard form, this is `x^2 - tx + y = 0`, with constants
    /// - `a = 1`
    /// - `b = -t`
    /// - `c = y`
    ///
    /// We add one to the lower solution and subtract one from the upper solution
    /// to handle the cases where the solutions are integers (where we'd tie the
    /// record distance and not beat it), then floor/ceil them to the nearest int.
    ///
    /// We also add one to make the range inclusive.
    pub fn ways_to_win(&self) -> u64 {
        let (x1, x2) = solve_quadratic(1.0, -(self.max_time as f64), self.record_distance as f64);
        let (x1, x2) = ((x1 + 1.0).floor(), (x2 - 1.0).ceil());
        x2 as u64 - x1 as u64 + 1
    }
}
