use aoc2021::prelude::*;

fn main() {
    let mut position = 0;
    let mut depth = 0;

    for line in parsed_lines::<String>() {
        let (command, count) = match line.split_once(' ') {
            Some(x) => x,
            None => continue,
        };

        let count: isize = count.parse().unwrap();

        match command {
            "forward" => position += count,
            "down" => depth += count,
            "up" => depth -= count,
            other => panic!("bad command {:?}", other),
        }
    }

    println!("{}", position * depth);
}
