use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

mod battery_bank;
use crate::battery_bank::BatteryBank;

pub mod file_parser;

pub fn solve(input_file: &str) -> (u64, u64) {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let result1 = Arc::new(AtomicU64::new(0));
    let result2 = Arc::new(AtomicU64::new(0));

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    for battery_bank in battery_banks {
        let result1_clone = Arc::clone(&result1);
        let result2_clone = Arc::clone(&result2);
        let handle = thread::spawn(move || {
            result1_clone.fetch_add(battery_bank.get_high_joltage(2), Ordering::SeqCst);
            result2_clone.fetch_add(battery_bank.get_high_joltage(12), Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked!");
    }

    (
        result1.load(Ordering::Relaxed),
        result2.load(Ordering::Relaxed),
    )
}

pub fn solve_pt1(input_file: &str) -> u64 {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let result1 = Arc::new(AtomicU64::new(0));

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    for battery_bank in battery_banks {
        let result1_clone = Arc::clone(&result1);
        let handle = thread::spawn(move || {
            result1_clone.fetch_add(battery_bank.get_high_joltage(2), Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked!");
    }

    result1.load(Ordering::Relaxed)
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let result2 = Arc::new(AtomicU64::new(0));

    let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(battery_banks.len());
    for battery_bank in battery_banks {
        let result2_clone = Arc::clone(&result2);
        let handle = thread::spawn(move || {
            result2_clone.fetch_add(battery_bank.get_high_joltage(12), Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked!");
    }

    result2.load(Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_parser::FileParser;

    #[test]
    fn example() {
        let file = FileParser::new("data/example.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 357);
        assert_eq!(part_2, 3121910778619);
    }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 357);
        assert_eq!(solve_pt2(my_file.get_str()), 3121910778619);
    }

    #[test]
    fn actual() {
        let file = FileParser::new("data/input.txt");
        let (part_1, part_2) = solve(file.get_str());
        assert_eq!(part_1, 17316);
        assert_eq!(part_2, 171741365473332);
    }

    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), 17316);
        assert_eq!(solve_pt2(my_file.get_str()), 171741365473332);
    }
}
