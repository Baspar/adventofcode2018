use regex::Regex;
use std::collections::HashMap;

// Types
#[derive(Debug, Copy, Clone)]
struct Date {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    min: u64
}
#[derive(Debug, Copy, Clone)]
struct SleepPeriod {
    guard: u64,
    from_date: Date,
    to_date: Date
}
impl SleepPeriod {
    pub fn min_elapsed (&self) -> u64 {
        (60 * self.to_date.hour + self.to_date.min) - (60 * self.from_date.hour + self.from_date.min)
    }
}

// Helper
fn parse_input(input: &str) -> Vec<Vec<SleepPeriod>> {
    let regexp = Regex::new(r"(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}).*(asleep|wakes|#(\d+))").unwrap();
    let mut out: Vec<Vec<SleepPeriod>> = Vec::new();

    // Sort input
    let mut lines: Vec<_> = input.lines().collect();
    lines.sort_unstable();

    let mut vec_buffer = Vec::new();
    let mut guard: u64 = 0;
    let mut from_date: Date = Date{ year: 0, month: 0, day: 0, hour: 0, min: 0 };

    for line in lines {
        // Match line
        let cap = regexp.captures_iter(line).next().unwrap();

        // Capture data
        let year: u64 = (&cap[1]).parse().unwrap();
        let month: u64 = (&cap[2]).parse().unwrap();
        let day: u64  = (&cap[3]).parse().unwrap();
        let hour: u64  = (&cap[4]).parse().unwrap();
        let min: u64 = (&cap[5]).parse().unwrap();
        let event_type = &cap[6];

        // Build event
        let date = Date { year, month, day, hour, min };

        if event_type == "asleep" {
            // Add new sleep event
            from_date = date;
        } else if event_type == "wakes" {
            // Add new awake event
            vec_buffer.push(SleepPeriod{ guard, from_date, to_date: date })
        } else {
            // If past event recorded, push them to out, clear buffer and set new guard
            if vec_buffer.len() > 0 {
                out.push(vec_buffer.to_vec());
                vec_buffer.clear();
                guard = (&cap[7]).parse().unwrap();
            }
        }
    }

    out
}

// Part1
pub fn part1 (input: &str) -> String {
    let shifts = parse_input(input);

    let mut total_sleep: HashMap<u64, u64> = HashMap::new();
    let mut minute_frequencies: HashMap<u64, HashMap<u64, u64>> = HashMap::new();
    for shift in shifts {
        for sleep_period in shift {
            // Add total sleep time per guard
            let iter1 = total_sleep.entry(sleep_period.guard).or_insert(0);
            *iter1 += sleep_period.min_elapsed();

            // Add minute slept per guard
            for minute in sleep_period.from_date.min..sleep_period.to_date.min {
                let iter2 = minute_frequencies.entry(sleep_period.guard).or_insert(HashMap::new());
                let iter3 = iter2.entry(minute).or_insert(0);
                *iter3 += 1;
            }
        }
    }

    // Find best guard
    let mut best_guard: u64 = 0;
    let mut guard_sleep: u64 = 0;
    for (guard, sleep_time) in total_sleep.iter() {
        if guard_sleep < *sleep_time {
            guard_sleep = *sleep_time;
            best_guard = *guard;
        }
    }

    // Find best min
    let mut best_min: u64 = 0;
    let mut best_freq:u64 = 0;
    let guard_freq = minute_frequencies.entry(best_guard).or_insert(HashMap::new());
    for (min, freq) in guard_freq.iter() {
        if best_freq < *freq {
            best_freq = *freq;
            best_min = *min;
        }
    }

    format!("{}", best_guard * best_min)
}

// Part2
pub fn part2 (input: &str) -> String {
    format!("input")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day4_part1 () {
        assert_eq!(super::part1("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"), 240);
    }

    #[test]
    fn day4_part2 () {
        assert_eq!(super::part2("[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"), 4455);
    }
}
