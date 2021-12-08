use aoc2021::prelude::*;

fn main() {
    let mut easy = 0;

    for line in parsed_lines::<String>() {
        let (_all, display) = line.split_once(" | ").unwrap();

        let display_digits = display.split_whitespace();

        for digit in display_digits {
            match digit.trim().len() {
                2 | 4 | 3 | 7 => easy += 1,
                _ => {}
            }
        }
    }

    println!("{}", easy);
}
