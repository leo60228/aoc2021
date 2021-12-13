use aoc2021::prelude::*;
use std::collections::{HashMap, HashSet};

fn search<'a>(
    cave: &'a str,
    system: &HashMap<&'a str, Vec<&'a str>>,
    mut explored: HashSet<&'a str>,
    mut history: Vec<&'a str>,
    doubled: bool,
    paths: &mut HashSet<Vec<&'a str>>,
) {
    history.push(cave);

    if cave == "start" {
        return;
    } else if cave == "end" {
        paths.insert(history);
        return;
    } else if cave.chars().next().unwrap().is_lowercase() {
        if explored.contains(&cave) {
            return;
        }

        if !doubled {
            for &next in &system[&cave] {
                search(next, system, explored.clone(), history.clone(), true, paths);
            }
        }

        explored.insert(cave);
    }

    for &next in &system[&cave] {
        search(
            next,
            system,
            explored.clone(),
            history.clone(),
            doubled,
            paths,
        );
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

    let mut paths = HashSet::new();

    for cave in &system["start"] {
        search(
            cave,
            &system,
            HashSet::new(),
            vec!["start"],
            false,
            &mut paths,
        );
    }

    println!("{}", paths.len());
}
