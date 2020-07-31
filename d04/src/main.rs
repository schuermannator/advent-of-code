use std::collections::HashMap;
use std::io::{self, Read};

use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

enum LogLine {
    GuardChange(u32),
    Sleep(u8),
    Wake(u8),
}

// keep hashtable of vecs per guard
fn main() -> Result<()> {
    let mut b = String::new();
    io::stdin().read_to_string(&mut b)?;

    p1(&b)?;
    p2(&b)?;

    Ok(())
}

enum SleepState {
    Awake,
    Asleep(u8),
    Nothing,
}

struct State {
    current_guard: Option<u32>,
    sleep_state: SleepState,
    sleep_logs: HashMap<u32, [u8; 60]>,
}

impl State {
    fn new() -> Self {
        State {
            current_guard: None,
            sleep_state: SleepState::Nothing,
            sleep_logs: HashMap::new(),
        }
    }

    fn guard_change(&mut self, new_guard: u32) {
        self.current_guard = Some(new_guard);
        self.sleep_state = SleepState::Awake;
    }

    fn start_sleep(&mut self, time: u8) {
        self.sleep_state = SleepState::Asleep(time);
    }

    fn wake_up(&mut self, end_sleep: u8) {
        if let Some(guard) = self.current_guard {
            if let SleepState::Asleep(start_sleep) = self.sleep_state {
                let log = self.sleep_logs.entry(guard).or_insert_with(|| [0; 60]);
                for i in start_sleep..end_sleep {
                    log[i as usize] += 1;
                }
            }
        }
    }

    fn get_maxes(&self) -> (u32, u8) {
        let mut max = 0;
        let mut guard = 0;
        for (k, v) in &self.sleep_logs {
            let s = v.iter().map(|&x| x as u32).sum();
            if s > max {
                max = s;
                guard = *k;
            }
        }
        let mut other_max = 0;
        let mut min = 0;
        for (i, v) in self.sleep_logs[&guard].iter().enumerate() {
            if v > &other_max {
                min = i;
                other_max = *v;
            }
        }
        (guard, min as u8)
    }

    fn get_frequent(&self) -> (u32, u8) {
        let mut max = 0;
        let mut guard = 0;
        for (k, v) in &self.sleep_logs {
            let s = v.iter().max().unwrap();
            if s > &max {
                max = *s;
                guard = *k;
            }
        }
        let mut other_max = 0;
        let mut min = 0;
        for (i, v) in self.sleep_logs[&guard].iter().enumerate() {
            if v > &other_max {
                min = i;
                other_max = *v;
            }
        }
        (guard, min as u8)
    }
}

fn p1(buf: &str) -> Result<()> {
    let mut state = State::new();
    // sort input
    let mut lines: Vec<&str> = buf.lines().collect();
    lines.sort();
    for line in lines {
        let log_line = parse_line(line)?;
        match log_line {
            LogLine::GuardChange(guard) => state.guard_change(guard),
            LogLine::Sleep(time) => state.start_sleep(time),
            LogLine::Wake(time) => state.wake_up(time),
        }
    }
    let (g, m) = state.get_maxes();
    println!("guard: {}, minute: {}", g, m);
    println!("{}", g * m as u32);
    Ok(())
}

fn p2(buf: &str) -> Result<()> {
    let mut state = State::new();
    // sort input
    let mut lines: Vec<&str> = buf.lines().collect();
    lines.sort();
    for line in lines {
        let log_line = parse_line(line)?;
        match log_line {
            LogLine::GuardChange(guard) => state.guard_change(guard),
            LogLine::Sleep(time) => state.start_sleep(time),
            LogLine::Wake(time) => state.wake_up(time),
        }
    }

    let (g, m) = state.get_frequent();
    println!("guard: {}, minute: {}", g, m);
    println!("{}", g * m as u32);
    Ok(())
}

fn parse_line(line: &str) -> Result<LogLine> {
    // TODO move regex compile outside of loop
    let guard_re = Regex::new(r"#(\d+)")?;
    let minute_re = Regex::new(r":(\d\d)")?;
    let minute: u8 = minute_re.captures(line).unwrap()[1].parse()?;
    if let Some(guard) = guard_re.captures(line) {
        Ok(LogLine::GuardChange(guard[1].parse()?))
    } else if line.contains("wakes up") {
        Ok(LogLine::Wake(minute))
    } else if line.contains("falls asleep") {
        Ok(LogLine::Sleep(minute))
    } else {
        Err(format!("unable to parse line: {}", line).into())
    }
}
