use std::cmp::Ordering;

use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub enum Log {
    ShiftBegins(i32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
pub struct Entry {
    timestamp: DateTime<Utc>,
    log: Log,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool { self.timestamp.eq(&other.timestamp) }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering { self.timestamp.cmp(&other.timestamp) }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_line(line: &str) -> Option<Entry> {
    lazy_static! {
        static ref LINE_MATCHER: Regex = Regex::new(r"\[([\d :-]{16})\] (Guard #(\d+) begins shift|falls asleep|wakes up)").unwrap();
    }

    LINE_MATCHER
        .captures(line)
        .map(|matches| {
            let timestamp_str = matches.get(1).unwrap().as_str();
            let timestamp_parsed = Utc.datetime_from_str(timestamp_str, "%Y-%m-%d %H:%M").unwrap();
            let log = match matches.get(2).map(|m| m.as_str()) {
                Some("falls asleep") => Some(Log::FallsAsleep),
                Some("wakes up") => Some(Log::WakesUp),
                Some(_) => Some(Log::ShiftBegins(matches.get(3).unwrap().as_str().parse().unwrap())),
                _ => None
            };

            Entry {
                timestamp: timestamp_parsed,
                log: log.unwrap(),
            }
        })
}

pub fn parse_logs(lines: &Vec<String>) -> Vec<Entry> {
    lines
        .iter()
        .map(|c| parse_line(c.as_str()))
        .flatten()
        .collect()
}

pub fn part_1(logs: &Vec<Entry>) -> i32 {
    !unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_claim() {
        assert_eq!(
            Some(
                Entry {
                    timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 0, 0),
                    log: Log::ShiftBegins(10),
                }
            ),
            parse_line("[1518-11-01 00:00] Guard #10 begins shift")
        );
        assert_eq!(
            Some(
                Entry {
                    timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 5, 0),
                    log: Log::FallsAsleep,
                }
            ),
            parse_line("[1518-11-01 00:05] falls asleep")
        );
        assert_eq!(
            Some(
                Entry {
                    timestamp: Utc.ymd(1518, 11, 01).and_hms(0, 25, 0),
                    log: Log::WakesUp,
                }
            ),
            parse_line("[1518-11-01 00:25] wakes up")
        );

        assert_eq!(None, parse_line(""))
    }
}

