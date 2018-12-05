use chrono::prelude::*;
use chrono::Duration;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

type Id = u32;

const DSTRING: &str = "%Y-%m-%d %H:%M";

fn main() {
    lazy_static! {
        static ref GUARD: Regex = Regex::new(r"\[(.*)\] Guard \#(\d+) begins shift").unwrap();
        static ref SLEEPS: Regex = Regex::new(r"\[(.*)\] falls asleep").unwrap();
        static ref WAKES: Regex = Regex::new(r"\[(.*)\] wakes up").unwrap();
    }
    let one_minute: Duration = Duration::minutes(1);

    let input = include_str!("../input");

    let mut sleeps: HashMap<Id, HashMap<u32, u32>> = HashMap::new();

    let mut id: Id = 0;
    let mut start_sleep = Utc.ymd(1500, 1, 1).and_hms(0, 0, 0);

    for line in input.lines() {
        if let Some(start) = GUARD.captures(line) {
            id = start[2].parse().unwrap();
        }

        if let Some(sleep) = SLEEPS.captures(line) {
            start_sleep = Utc.datetime_from_str(&sleep[1], DSTRING).unwrap();
        }

        if let Some(wake) = WAKES.captures(line) {
            let end = Utc.datetime_from_str(&wake[1], DSTRING).unwrap();
            let mut t = start_sleep.clone();
            while t != end {
                let minute: u32 = t.format("%M").to_string().parse().unwrap();
                *sleeps
                    .entry(id)
                    .or_insert(HashMap::new())
                    .entry(minute)
                    .or_insert(0) += 1;
                t = t.checked_add_signed(one_minute).unwrap();
            }
        }
    }

    let mut max_sleep = 0;
    let mut big_sleeper = 0;
    let mut most_sleepy_minute = 0;
    let mut minute_guard = 0;
    let mut max_count = 0;
    for (k, v) in sleeps.iter() {
        let mut s = 0;
        for (m, c) in v.iter() {
            s += c;
            if *c > max_count {
                max_count = *c;
                most_sleepy_minute = *m;
                minute_guard = *k;
            }
        }
        if s > max_sleep {
            max_sleep = s;
            big_sleeper = *k;
        }
    }

    let mut sleepiest_minute = 0;
    let mut count = 0;

    for (k, v) in sleeps[&big_sleeper].iter() {
        if *v > count {
            count = *v;
            sleepiest_minute = *k;
        }
    }

    // part 1 answer:
    println!(
        "Guard {} sleeps most at {}, for a checksum of {}",
        big_sleeper,
        sleepiest_minute,
        big_sleeper * sleepiest_minute
    );

    println!(
        "Guard {} sleeps at minute {} more than any other guard. Checksum is {}.",
        minute_guard,
        most_sleepy_minute,
        minute_guard * most_sleepy_minute
    );
}
