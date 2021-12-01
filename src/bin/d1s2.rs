use itertools::Itertools;
use std::io::{prelude::*, stdin};

fn main() {
    let input: Vec<usize> = stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|x| x.parse())
        .collect();

    let count = input
        .windows(3)
        .map(|x| x.iter().sum::<usize>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    println!("{}", count);
}
