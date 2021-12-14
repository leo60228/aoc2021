use aoc2021::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut lines = lines();

    let template: Vec<_> = lines.next().unwrap().bytes().collect();
    let mut rules = HashMap::new();

    for line in lines.skip(1) {
        let (a, b) = line.split_once(" -> ").unwrap();
        let a: Vec<_> = a.bytes().collect();
        assert_eq!(a.len(), 2);
        let b = b.bytes().exactly_one().unwrap();
        rules.insert((a[0], a[1]), b);
    }

    let mut counts = template.iter().copied().counts();
    let mut pair_counts = template.iter().copied().tuple_windows::<(_, _)>().counts();
    let mut insertions = Vec::new();

    for i in 0..40 {
        println!("iteration: {}", i);

        for (&(a, b), &insertion) in &rules {
            if let Some(count) = pair_counts.insert((a, b), 0) {
                insertions.push((a, b, insertion, count));
            }
        }

        for (a, b, insertion, count) in insertions.drain(..) {
            *counts.entry(insertion).or_insert(0) += count;
            *pair_counts.entry((a, insertion)).or_insert(0) += count;
            *pair_counts.entry((insertion, b)).or_insert(0) += count;
        }
    }

    let least = counts.values().min().unwrap();
    let most = counts.values().max().unwrap();

    println!("{}", most - least);
}
