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

    pub fn get_high_joltage(&self) -> u8 {
        let mut first = 0u8;
        let mut second = 0u8;

        for i in 0..self.batteries.len() - 1 {
            let curr_value = self.batteries[i];

            if curr_value > first {
                first = curr_value;
                second = self.batteries[i + 1];
            } else if curr_value > second {
                second = curr_value
            }
        }

        {
            let last_val = *self.batteries.last().unwrap();
            if last_val > second {
                second = last_val;
            }
        }

        println!("{}", first * 10 + second);
        first * 10 + second
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
        assert_eq!(bank.get_high_joltage(), 98);

        bank = BatteryBank::new("811111111111119");
        assert_eq!(bank.get_high_joltage(), 89);

        bank = BatteryBank::new("234234234234278");
        assert_eq!(bank.get_high_joltage(), 78);

        bank = BatteryBank::new("818181911112111");
        assert_eq!(bank.get_high_joltage(), 92);
    }
}
