use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

mod file_parser;
use crate::file_parser::{FileParser, FileParserTrait};

pub fn solve(input_file: &str) -> (u64, u64) {
    let _battery_banks = FileParser::new(input_file).parse_lines();

    let result1 = Arc::new(AtomicU64::new(0));
    let result2 = Arc::new(AtomicU64::new(0));

    // let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::with_capacity(id_range_vector.len());
    // for id_range in id_range_vector {
    //     let result1_clone = Arc::clone(&result1);
    //     let result2_clone = Arc::clone(&result2);
    //     let handle = thread::spawn(move || {
    //         let (invalid_ids_1, invalid_ids_2) = id_range.get_invalid_ids();
    //         for invalid_id in invalid_ids_1 {
    //             result1_clone.fetch_add(invalid_id, Ordering::SeqCst);
    //         }
    //         for invalid_id in invalid_ids_2 {
    //             result2_clone.fetch_add(invalid_id, Ordering::SeqCst);
    //         }
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().expect("Thread panicked!");
    // }

    (
        result1.load(Ordering::Relaxed),
        result2.load(Ordering::Relaxed),
    )
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn example() {
    //     let (part_1, part_2) = solve("data/example.txt");
    //     assert_eq!(part_1, 357);
    //     assert_eq!(part_2, 4174379265);
    // }

    // #[test]
    // fn actual() {
    //     let (part_1, part_2) = solve("data/input.txt");
    //     assert_eq!(part_1, 13919717792);
    //     assert_eq!(part_2, 14582313461);
    // }
}
