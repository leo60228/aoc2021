use aoc2021::prelude::*;
use std::collections::HashSet;

const REAL_DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn main() {
    let mut sum = 0;

    for line in parsed_lines::<String>() {
        let (all, display) = line.split_once(" | ").unwrap();

        let all_digits: Vec<HashSet<_>> = all
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().map(|y| (y as u8) - b'a').collect())
            .collect();

        let mut orders = (0..7).permutations(7).filter(|order| {
            let mut digits = all_digits.clone();

            for digit in REAL_DIGITS {
                let hashset: HashSet<_> = digit
                    .chars()
                    .map(|x| order[((x as u8) - b'a') as usize])
                    .collect();
                if let Some((idx, _)) = digits.iter().enumerate().find(|(_, x)| x == &&hashset) {
                    digits.remove(idx);
                } else {
                    return false;
                }
            }

            true
        });

        let order = orders.next().unwrap();
        assert_eq!(orders.next(), None);

        let digits: Vec<_> = display
            .split_whitespace()
            .map(|digit| {
                let fixed_segments: HashSet<_> = digit
                    .chars()
                    .map(|x| {
                        let broken = (x as u8) - b'a';
                        let (fixed, _) = order
                            .iter()
                            .copied()
                            .enumerate()
                            .find(|&(_, y)| y == broken)
                            .unwrap();
                        fixed as u8
                    })
                    .collect();

                let (digit, _) = REAL_DIGITS
                    .iter()
                    .enumerate()
                    .find(|(_, x)| {
                        let hashset: HashSet<_> = x.chars().map(|x| (x as u8) - b'a').collect();
                        hashset == fixed_segments
                    })
                    .unwrap();

                digit
            })
            .collect();

        let num = digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];

        sum += num;
    }

    println!("{}", sum);
}
