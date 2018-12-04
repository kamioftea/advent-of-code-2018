use std::cmp::Ordering;
use std::collections::HashMap;

use chrono::{DateTime, Timelike, TimeZone, Utc};
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
    let mut entries: Vec<Entry> = lines
        .iter()
        .map(|c| parse_line(c.as_str()))
        .flatten()
        .collect();

    entries.sort();

    entries
}

fn build_guards(logs: &Vec<Entry>) -> HashMap<i32, HashMap<u32, u32>> {
    let mut guards: HashMap<i32, HashMap<u32, u32>> = HashMap::new();
    let mut current_guard_id = -1;
    let mut start_minute: u32 = 0;

    for entry in logs {
        match entry.log {
            Log::ShiftBegins(id) => current_guard_id = id,
            Log::FallsAsleep => start_minute = entry.timestamp.minute(),
            Log::WakesUp => {
                let mut guard = guards.entry(current_guard_id).or_insert(HashMap::new());
                for min in start_minute..entry.timestamp.minute() {
                    *guard.entry(min).or_insert(0) += 1;
                }
            }
        }
    }

    guards
}

pub fn part_1(logs: &Vec<Entry>) -> (i32, u32) {
    let guards = build_guards(logs);

    let (&id, _) =
        guards
            .iter()
            .map(|(id, minutes)| (id, minutes.values().fold(0, |acc, v| acc + v)))
            .max_by(|(_, a), (_, b)| a.cmp(&b)).unwrap();

    let (&minute, _) = guards.get(&id).unwrap().iter().max_by(|(_, a), (_, b)| a.cmp(&b)).unwrap();

    (id, minute)
}

pub fn part_2(logs: &Vec<Entry>) -> (i32, u32) {
    let guards = build_guards(logs);

    let (&id, (&minute, _)) =
        guards
            .iter()
            .map(|(id, minutes)| (id, minutes.iter().max_by(|(_, a),(_,b)| a.cmp(&b)).unwrap()))
            .max_by(|(_, (_,a)),(_,(_,b))| a.cmp(&b)).unwrap();

    (id, minute)
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

    #[test]
    fn test_part_1() {
        assert_eq!(
            (10, 24),
            part_1(
                &parse_logs(
                    &vec![
                        "[1518-11-01 00:00] Guard #10 begins shift",
                        "[1518-11-01 00:05] falls asleep",
                        "[1518-11-01 00:25] wakes up",
                        "[1518-11-01 00:30] falls asleep",
                        "[1518-11-01 00:55] wakes up",
                        "[1518-11-01 23:58] Guard #99 begins shift",
                        "[1518-11-02 00:40] falls asleep",
                        "[1518-11-02 00:50] wakes up",
                        "[1518-11-03 00:05] Guard #10 begins shift",
                        "[1518-11-03 00:24] falls asleep",
                        "[1518-11-03 00:29] wakes up",
                        "[1518-11-04 00:02] Guard #99 begins shift",
                        "[1518-11-04 00:36] falls asleep",
                        "[1518-11-04 00:46] wakes up",
                        "[1518-11-05 00:03] Guard #99 begins shift",
                        "[1518-11-05 00:45] falls asleep",
                        "[1518-11-05 00:55] wakes up"
                    ].iter().map(|x| x.to_string()).collect()
                )
            )
        );

        assert_eq!(
            (10, 24),
            part_1(
                &parse_logs(
                    &vec![
                        "[1518-11-01 00:05] falls asleep",
                        "[1518-11-01 00:25] wakes up",
                        "[1518-11-01 00:30] falls asleep",
                        "[1518-11-02 00:40] falls asleep",
                        "[1518-11-05 00:03] Guard #99 begins shift",
                        "[1518-11-01 00:00] Guard #10 begins shift",
                        "[1518-11-02 00:50] wakes up",
                        "[1518-11-03 00:05] Guard #10 begins shift",
                        "[1518-11-03 00:29] wakes up",
                        "[1518-11-05 00:45] falls asleep",
                        "[1518-11-01 00:55] wakes up",
                        "[1518-11-04 00:02] Guard #99 begins shift",
                        "[1518-11-04 00:36] falls asleep",
                        "[1518-11-03 00:24] falls asleep",
                        "[1518-11-01 23:58] Guard #99 begins shift",
                        "[1518-11-04 00:46] wakes up",
                        "[1518-11-05 00:55] wakes up"
                    ].iter().map(|x| x.to_string()).collect()
                )
            )
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            (99, 45),
            part_2(
                &parse_logs(
                    &vec![
                        "[1518-11-01 00:00] Guard #10 begins shift",
                        "[1518-11-01 00:05] falls asleep",
                        "[1518-11-01 00:25] wakes up",
                        "[1518-11-01 00:30] falls asleep",
                        "[1518-11-01 00:55] wakes up",
                        "[1518-11-01 23:58] Guard #99 begins shift",
                        "[1518-11-02 00:40] falls asleep",
                        "[1518-11-02 00:50] wakes up",
                        "[1518-11-03 00:05] Guard #10 begins shift",
                        "[1518-11-03 00:24] falls asleep",
                        "[1518-11-03 00:29] wakes up",
                        "[1518-11-04 00:02] Guard #99 begins shift",
                        "[1518-11-04 00:36] falls asleep",
                        "[1518-11-04 00:46] wakes up",
                        "[1518-11-05 00:03] Guard #99 begins shift",
                        "[1518-11-05 00:45] falls asleep",
                        "[1518-11-05 00:55] wakes up"
                    ].iter().map(|x| x.to_string()).collect()
                )
            )
        );

        assert_eq!(
            (99, 45),
            part_2(
                &parse_logs(
                    &vec![
                        "[1518-11-01 00:05] falls asleep",
                        "[1518-11-01 00:25] wakes up",
                        "[1518-11-01 00:30] falls asleep",
                        "[1518-11-02 00:40] falls asleep",
                        "[1518-11-05 00:03] Guard #99 begins shift",
                        "[1518-11-01 00:00] Guard #10 begins shift",
                        "[1518-11-02 00:50] wakes up",
                        "[1518-11-03 00:05] Guard #10 begins shift",
                        "[1518-11-03 00:29] wakes up",
                        "[1518-11-05 00:45] falls asleep",
                        "[1518-11-01 00:55] wakes up",
                        "[1518-11-04 00:02] Guard #99 begins shift",
                        "[1518-11-04 00:36] falls asleep",
                        "[1518-11-03 00:24] falls asleep",
                        "[1518-11-01 23:58] Guard #99 begins shift",
                        "[1518-11-04 00:46] wakes up",
                        "[1518-11-05 00:55] wakes up"
                    ].iter().map(|x| x.to_string()).collect()
                )
            )
        )
    }
}

