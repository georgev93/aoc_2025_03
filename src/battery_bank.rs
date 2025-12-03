pub struct BatteryBank {
    batteries: Vec<u8>,
}

impl BatteryBank {
    pub fn new(input_string: &str) -> Self {
        Self {
            batteries: input_string
                .chars()
                .map(|b| b.to_digit(10).unwrap() as u8)
                .collect(),
        }
    }

    pub fn get_high_joltage(&self, num_of_batteries: usize) -> u64 {
        let mut total_vec: Vec<u8> = vec![];

        Self::get_highest_possible_from_slice(&self.batteries, &mut total_vec, num_of_batteries);

        total_vec.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
    }

    fn get_highest_possible_from_slice(
        input: &[u8],
        total_vec: &mut Vec<u8>,
        num_of_batteries: usize,
    ) {
        let mut best_position = 0;
        let mut value_at_best_position = input[0];

        for position in 0..input.len() - num_of_batteries + 1 {
            let value_at_position = input[position];
            if value_at_position > value_at_best_position {
                best_position = position;
                value_at_best_position = value_at_position;
            }
        }
        total_vec.push(value_at_best_position);

        if num_of_batteries > 1 {
            // let mut new_slice = Vec::with_capacity(input.len() - best_position);
            // println!("testing {}", input.len() - best_position);
            // new_slice.copy_from_slice(&input[best_position..]);
            let new_slice = &input[(best_position + 1)..];
            Self::get_highest_possible_from_slice(new_slice, total_vec, num_of_batteries - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_instantiation() {
        let bank = BatteryBank::new("12345");
        assert_eq!(bank.batteries, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn high_joltage() {
        let mut bank = BatteryBank::new("987654321111111");
        assert_eq!(bank.get_high_joltage(2), 98);
        assert_eq!(bank.get_high_joltage(12), 987654321111);

        bank = BatteryBank::new("811111111111119");
        assert_eq!(bank.get_high_joltage(2), 89);
        assert_eq!(bank.get_high_joltage(12), 811111111119);

        bank = BatteryBank::new("234234234234278");
        assert_eq!(bank.get_high_joltage(2), 78);
        assert_eq!(bank.get_high_joltage(12), 434234234278);

        bank = BatteryBank::new("818181911112111");
        assert_eq!(bank.get_high_joltage(2), 92);
        assert_eq!(bank.get_high_joltage(12), 888911112111);
    }
}
