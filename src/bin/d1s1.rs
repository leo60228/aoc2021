#![feature(array_windows)]

use std::io::{prelude::*, stdin};

fn main() {
    let input: Vec<usize> = stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|x| x.parse())
        .collect();

    let count = input.array_windows().filter(|[a, b]| b > a).count();

    println!("{}", count);
}
