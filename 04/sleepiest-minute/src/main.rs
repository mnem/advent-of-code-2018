extern crate regex;
extern crate chrono;

use std::fs;
use regex::Regex;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;

fn main() {
    let input = read_input_lines();
    let result = process_lines(&input);
    println!("Result: {}\n", result);
}

fn read_input_lines() -> String {
    let input_filename = String::from("input.txt");
    fs::read_to_string(input_filename)
        .expect("Failed to read file")
}

fn process_lines(input: &str) -> u32 {
    let records = records_from_lines(input);
    let sleep_records = sleep_records_from(records);
    let guard_durations = sleep_durations_by_guard_from(sleep_records);

    let mut most_frequent_guard = 0;
    let mut most_frequent_minute = 0;
    let mut most_frequent_value = 0;
    for (guard, periods) in &guard_durations {
        let (sleepiest_minute, sleepiest_value) = sleepiest_minute(periods);
        if sleepiest_value > most_frequent_value {
            most_frequent_guard = *guard;
            most_frequent_minute = sleepiest_minute;
            most_frequent_value = sleepiest_value;
        }
    }

    return most_frequent_minute * (most_frequent_guard as u32);
}

fn sleepiest_minute(periods: &Vec<SleepPeriod>) -> (u32, i32) {
    let mut sleep_by_minute = HashMap::new();
    for period in periods {
        for minute in period.start.minute()..(period.start.minute() + period.duration.num_minutes() as u32) {
            let current = sleep_by_minute.entry(minute).or_insert(0);
            *current += 1;
        }
    }
    let mut sleepiest_minute = 0;
    let mut sleepiest_value = -1;
    for (minute, value) in sleep_by_minute {
        if value > sleepiest_value {
            sleepiest_minute = minute;
            sleepiest_value = value;
        }
    }
    (sleepiest_minute, sleepiest_value)
}

fn sleep_durations_by_guard_from(sleep_records: Vec<SleepRecord>) -> HashMap<i32, Vec<SleepPeriod>> {
    let mut durations_by_guard = HashMap::new();

    for record in &sleep_records {
        let durations = durations_by_guard.entry(record.guard).or_insert(Vec::new());
        durations.push(record.sleep);
    }

    return durations_by_guard;
}

#[derive(Debug,PartialEq)]
enum GuardState {
    Begin(i32),
    Awake,
    Asleep,
}

impl GuardState {
    fn from(string: &str) -> GuardState {
        let guard_re = Regex::new(r"#(?P<id>\d*)").unwrap();
        let guard_captures = guard_re.captures(string);
        if let Some(guard_captures) = guard_captures {
            return GuardState::Begin(guard_captures["id"].parse().unwrap());
        } else {
            if string == "falls asleep" {
                return GuardState::Asleep;
            } else if string == "wakes up" {
                return GuardState::Awake;
            } else {
                panic!("Unknown state string! '{}'", string);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct SleepPeriod {
    start: DateTime<Utc>,
    duration: Duration,
}

#[derive(Debug)]
struct SleepRecord {
    guard: i32,
    sleep: SleepPeriod,
}

#[derive(Debug)]
struct Record {
    moment: DateTime<Utc>,
    guard: i32,
    state: GuardState,
}

fn split_entry(entry_string: &str) -> (DateTime<Utc>, GuardState) {
    let re = Regex::new(r"\[(?P<date>.*)\] (?P<entry>.*)").unwrap();
    let captures = re.captures(entry_string)
        .expect("Unexpected entry format");
    let date = date_from(&captures["date"]);
    let state = GuardState::from(&captures["entry"]);
    (date, state)
}

fn date_from(string: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(string, "%Y-%m-%d %H:%M")
        .expect("Unexpected date format")
}

fn records_from_lines(text: &str) -> Vec<Record> {
    let mut records = Vec::new();

    // Initial parse
    for line in text.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }

        let (moment, state) = split_entry(line);
        records.push(Record { moment, guard: -1, state });
    }
    records.sort_by( |a, b| { a.moment.cmp(&b.moment) });

    // Fixup the IDs
    let mut current_guard = -1;
    for record in &mut records {
        match record.state {
            GuardState::Begin(guard_id) => current_guard = guard_id,
            _ => assert_ne!(-1, current_guard),
        }
        record.guard = current_guard;
    }

    return records;
}

fn sleep_records_from(records: Vec<Record>) -> Vec<SleepRecord> {
    let mut sleep_records = Vec::new();

    let mut sleep_start = None;
    for record in &records {

        match record.state {
            GuardState::Asleep => {
                match sleep_start {
                    None => sleep_start = Some(record),
                    _ => panic!("More than one guard is falling asleep!"),
                }
            },
            _ => {
                // If someone is waking up then record the record
                if let Some(start) = sleep_start {
                    assert_eq!(start.guard, record.guard);
                    let duration = record.moment.signed_duration_since(start.moment);
                    sleep_start = None;
                    sleep_records.push(SleepRecord {guard: record.guard, sleep: SleepPeriod { start: start.moment, duration }});
                }

            }
        }
    }

    return sleep_records;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up\n";
        let result = process_lines(lines);
        assert_eq!(4455, result);
    }

    #[test]
    fn test_sleep_durations() {
        let lines = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n";
        let records = records_from_lines(lines);
        let result = sleep_records_from(records);

        assert_eq!(3, result.len());

        assert_eq!(10, result[0].guard);
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(0, 5, 0), result[0].sleep.start);
        assert_eq!(Duration::minutes(20), result[0].sleep.duration);

        assert_eq!(10, result[1].guard);
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(0, 30, 0), result[1].sleep.start);
        assert_eq!(Duration::minutes(25), result[1].sleep.duration);

        assert_eq!(99, result[2].guard);
        assert_eq!(Utc.ymd(1518, 11, 2).and_hms(0, 40, 0), result[2].sleep.start);
        assert_eq!(Duration::minutes(10), result[2].sleep.duration);
    }

    #[test]
    fn test_sleep_durations_by_guard() {
        let lines = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n";
        let records = records_from_lines(lines);
        let sleep_records = sleep_records_from(records);
        let result = sleep_durations_by_guard_from(sleep_records);

        assert_eq!(2, result.len());

        assert_eq!(2, result[&10].len());
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(0, 5, 0), result[&10][0].start);
        assert_eq!(Duration::minutes(20), result[&10][0].duration);
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(0, 30, 0), result[&10][1].start);
        assert_eq!(Duration::minutes(25), result[&10][1].duration);

        assert_eq!(1, result[&99].len());
        assert_eq!(Utc.ymd(1518, 11, 2).and_hms(0, 40, 0), result[&99][0].start);
        assert_eq!(Duration::minutes(10), result[&99][0].duration);
    }

    #[test]
    fn test_lines_to_records() {
        let lines = "[1518-11-01 00:05] falls asleep\n[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n";
        let result = records_from_lines(lines);

        assert_eq!(7, result.len());
        assert_eq!(10, result[0].guard);
        assert_eq!(10, result[1].guard);
        assert_eq!(10, result[2].guard);
        assert_eq!(10, result[3].guard);
        assert_eq!(10, result[4].guard);
        assert_eq!(99, result[5].guard);
        assert_eq!(99, result[6].guard);
    }

    #[test]
    fn test_split_entry() {
        let (date, state) = split_entry("[1518-11-01 00:00] Guard #10 begins shift");
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(0, 0, 0), date);
        assert_eq!(GuardState::Begin(10), state);

        let (date, state) = split_entry("[1518-11-01 01:02] falls asleep");
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(1, 2, 0), date);
        assert_eq!(GuardState::Asleep, state);

        let (date, state) = split_entry("[1518-11-01 03:04] wakes up");
        assert_eq!(Utc.ymd(1518, 11, 1).and_hms(3, 4, 0), date);
        assert_eq!(GuardState::Awake, state);
    }

    #[test]
    fn test_guard_state_string_with_id() {
        let state = GuardState::from("Guard #10 begins shift");
        assert_eq!(GuardState::Begin(10), state);
    }

    #[test]
    fn test_guard_state_string_asleep() {
        let state = GuardState::from("falls asleep");
        assert_eq!(GuardState::Asleep, state);
    }

    #[test]
    fn test_guard_state_string_awake() {
        let state = GuardState::from("wakes up");
        assert_eq!(GuardState::Awake, state);
    }

    #[test]
    fn test_date_parsing() {
        let date = date_from("1518-11-01 00:05");
        assert_eq!(1518, date.year());
        assert_eq!(11, date.month());
        assert_eq!(1, date.day());

        assert_eq!(0, date.hour());
        assert_eq!(5, date.minute());
        assert_eq!(0, date.second());
    }
}
