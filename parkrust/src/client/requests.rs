use chrono::Duration;
use std::{collections::HashSet, time::Duration as StdDuration};

use crate::models::parkrun::RunResult;

pub fn total_time(results: &[RunResult]) -> Duration {
    let seconds: i64 = results
        .iter()
        .map(|result| result.duration().num_seconds())
        .sum();
    Duration::from_std(StdDuration::from_secs(seconds.try_into().unwrap())).unwrap()
}

pub fn average_time(results: &[RunResult]) -> Duration {
    total_time(results) / results.len().try_into().unwrap()
}

pub fn fastest_time(results: &[RunResult]) -> Duration {
    results
        .iter()
        .map(|result| result.duration())
        .min()
        .unwrap()
}

pub fn best_finishing_position(results: &[RunResult]) -> Duration {
    results
        .iter()
        .map(|result| result.duration())
        .min()
        .unwrap()
}

pub fn average_finishing_position(results: &[RunResult]) -> Duration {
    results
        .iter()
        .map(|result| result.duration())
        .min()
        .unwrap()
}

/// Return average speed. The result is the duration per km
pub fn average_speed(results: &[RunResult]) -> Duration {
    let num_seconds: i64 = results
        .iter()
        .map(|result| result.speed().num_seconds())
        .sum();
    let average_num_seconds: i64 = num_seconds / results.len() as i64;
    Duration::from_std(StdDuration::from_secs(average_num_seconds as u64)).unwrap()
}

pub fn events(results: &[RunResult]) -> HashSet<String> {
    HashSet::from_iter(results.iter().map(|result| result.event_number.clone()))
}

pub fn duration_formatter(duration: Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = (duration.num_seconds() / 60) / 60;

    if hours == 0 {
        format!("{:0>2}:{:0>2}", minutes, seconds)
    } else {
        format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    macro_rules! duration_test {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($expected, duration_formatter($input));
            }
        };
    }

    // duration_test!()
}
