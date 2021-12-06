use aoc2021::prelude::*;

fn main() {
    let mut lanternfish = split_line::<usize>(',');

    for _ in 0..80 {
        lanternfish = lanternfish
            .into_iter()
            .flat_map(|fish| {
                if fish == 0 {
                    [Some(6), Some(8)].into_iter().flatten()
                } else {
                    [Some(fish - 1), None].into_iter().flatten()
                }
            })
            .collect();
    }

    println!("{}", lanternfish.len());
}
