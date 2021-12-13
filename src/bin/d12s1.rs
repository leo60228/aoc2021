use aoc2021::prelude::*;
use std::collections::{HashMap, HashSet};

fn search<'a>(
    cave: &'a str,
    system: &HashMap<&'a str, Vec<&'a str>>,
    mut explored: HashSet<&'a str>,
    total: &mut usize,
) {
    if cave == "start" {
        return;
    } else if cave == "end" {
        *total += 1;
        return;
    } else if cave.chars().next().unwrap().is_lowercase() {
        if explored.contains(&cave) {
            return;
        }

        explored.insert(cave);
    }

    for &next in &system[&cave] {
        search(next, system, explored.clone(), total);
    }
}

fn main() {
    let lines = parsed_lines::<String>();
    let mut system = HashMap::new();

    for line in &lines {
        let (a, b) = line.split_once('-').unwrap();

        system.entry(a).or_insert_with(Vec::new).push(b);
        system.entry(b).or_insert_with(Vec::new).push(a);
    }

    let mut total = 0;

    for cave in &system["start"] {
        search(cave, &system, HashSet::new(), &mut total);
    }

    println!("{}", total);
}
