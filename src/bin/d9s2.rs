use aoc2021::prelude::*;
use std::cmp::Reverse;

fn find_target(
    floor: &[Vec<usize>],
    targets: &mut [Vec<Option<(usize, usize)>>],
    (x, y): (usize, usize),
) -> (usize, usize) {
    if let Some(target) = targets[y][x] {
        return target;
    }

    let height = floor[y][x];

    if height == 9 {
        return (x, y);
    }

    let neighbors = [
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
    ];

    let lowest = neighbors
        .into_iter()
        .filter(|&(x, y)| y < floor.len() && x < floor[y].len())
        .min_by_key(|&(x, y)| floor[y][x]);

    let target = if let Some((nx, ny)) = lowest {
        let nheight = floor[ny][nx];

        if nheight < height {
            find_target(floor, targets, (nx, ny))
        } else {
            (x, y)
        }
    } else {
        (x, y)
    };

    targets[y][x] = Some(target);

    target
}

fn main() {
    let lines = parsed_lines::<String>();
    let floor: Vec<Vec<usize>> = lines
        .into_iter()
        .map(|x| {
            x.chars()
                .filter_map(|y| y.to_digit(10))
                .map(|x| x as usize)
                .collect()
        })
        .collect();

    let mut targets = vec![vec![None; floor[0].len()]; floor.len()];

    for y in 0..floor.len() {
        for x in 0..floor[0].len() {
            if targets[y][x].is_some() {
                continue;
            }

            find_target(&floor, &mut targets, (x, y));
        }
    }

    let basins = targets.into_iter().flatten().flatten().counts();

    let largest = basins
        .values()
        .map(Reverse)
        .k_smallest(3)
        .fold(1, |a, Reverse(x)| a * x);

    println!("{}", largest);
}
