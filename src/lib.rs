mod battery_bank;
use crate::battery_bank::BatteryBank;

pub mod file_parser;

pub fn solve(input_file: &str) -> (u64, u64) {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let mut result1 = 0u64;
    let mut result2 = 0u64;

    for battery_bank in battery_banks {
        result1 += battery_bank.get_high_joltage(2);
        result2 += battery_bank.get_high_joltage(12);
    }

    (result1, result2)
}

pub fn solve_pt1(input_file: &str) -> u64 {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let mut result = 0u64;

    for battery_bank in battery_banks {
        result += battery_bank.get_high_joltage(2);
    }

    result
}

pub fn solve_pt2(input_file: &str) -> u64 {
    let input_lines: Vec<&str> = input_file.lines().collect();
    let battery_banks = input_lines.iter().map(|s| BatteryBank::new(s));

    let mut result = 0u64;

    for battery_bank in battery_banks {
        result += battery_bank.get_high_joltage(12);
    }

    result
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
