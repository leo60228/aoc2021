use aoc2021::prelude::*;

fn main() {
    let lanternfish = split_line::<u8>(',');
    let mut by_timer = lanternfish.into_iter().counts();

    for timer in 0..=8 {
        by_timer.entry(timer).or_insert(0);
    }

    for _ in 0..256 {
        let new = by_timer[&0];
        by_timer.insert(0, 0);

        for timer in 1..=8 {
            let count = by_timer[&timer];
            by_timer.insert(timer, 0);
            by_timer.insert(timer - 1, by_timer[&(timer - 1)] + count);
        }

        by_timer.insert(6, by_timer[&6] + new);
        by_timer.insert(8, by_timer[&8] + new);
    }

    println!("{}", sum(by_timer.values()));
}
