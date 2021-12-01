use aoc2021::prelude::*;

fn main() {
    let input: Vec<usize> = parsed_lines();
    let count = input.iter().tuple_windows().filter(|(a, b)| b > a).count();

    println!("{}", count);
}
