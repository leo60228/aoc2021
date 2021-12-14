use aoc2021::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut lines = lines();

    let mut template: Vec<_> = lines.next().unwrap().chars().collect();
    let mut rules = HashMap::new();

    for line in lines.skip(1) {
        let (a, b) = line.split_once(" -> ").unwrap();
        let a: Vec<_> = a.chars().collect();
        assert_eq!(a.len(), 2);
        let b = b.chars().exactly_one().unwrap();
        rules.insert((a[0], a[1]), b);
    }

    let mut insertions = Vec::new();

    for _ in 0..10 {
        insertions.clear();

        for (i, window) in template.iter().copied().tuple_windows().enumerate() {
            if let Some(&x) = rules.get(&window) {
                insertions.push((i + 1, x));
            }
        }

        for &(i, x) in insertions.iter().rev() {
            template.insert(i, x);
        }
    }

    let counts = template.iter().copied().counts();
    let least = counts.values().min().unwrap();
    let most = counts.values().max().unwrap();

    println!("{}", most - least);
}
