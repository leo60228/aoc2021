use aoc2021::prelude::*;

fn nth_triangular(n: isize) -> isize {
    n * (n + 1) / 2
}

fn main() {
    let crabs = split_line::<isize>(',');
    let max = crabs.iter().copied().max().unwrap();
    let fuel = (0..=max)
        .map(|x| sum(crabs.iter().map(|&y| nth_triangular((y - x).abs()))))
        .min()
        .unwrap();
    println!("{}", fuel);
}
