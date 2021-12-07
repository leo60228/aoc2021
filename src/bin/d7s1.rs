use aoc2021::prelude::*;

fn main() {
    let mut crabs = split_line::<isize>(',');
    crabs.sort_unstable();
    let target = crabs[crabs.len() / 2];
    let fuel = sum(crabs.iter().copied().map(|x| (x - target).abs()));
    println!("{}", fuel);
}
