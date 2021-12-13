use aoc2021::prelude::*;

fn flash(octopi: &mut [Vec<u8>], (x, y): (usize, usize), flashed: &mut Vec<(usize, usize)>) {
    if flashed.contains(&(x, y)) {
        return;
    }

    flashed.push((x, y));

    let x = x as isize;
    let y = y as isize;

    for x2 in [x - 1, x, x + 1] {
        if x2 < 0 || (x2 as usize) >= octopi[0].len() {
            continue;
        }

        for y2 in [y - 1, y, y + 1] {
            if y2 < 0 || (y2 as usize) >= octopi.len() {
                continue;
            }

            if (x, y) == (x2, y2) {
                continue;
            }

            increase(octopi, (x2 as usize, y2 as usize), flashed);
        }
    }
}

fn increase(octopi: &mut [Vec<u8>], (x, y): (usize, usize), flashed: &mut Vec<(usize, usize)>) {
    octopi[y][x] += 1;

    if octopi[y][x] > 9 {
        flash(octopi, (x, y), flashed);
    }
}

fn step(octopi: &mut [Vec<u8>]) -> usize {
    let mut flashed = Vec::new();

    let height = octopi.len();
    let width = octopi[0].len();

    for y in 0..height {
        for x in 0..width {
            increase(octopi, (x, y), &mut flashed);
        }
    }

    for &(x, y) in &flashed {
        octopi[y][x] = 0;
    }

    flashed.len()
}

fn main() {
    let rows = parsed_lines::<String>();
    let mut octopi: Vec<Vec<u8>> = rows
        .into_iter()
        .map(|x| {
            x.chars()
                .filter_map(|y| y.to_digit(10).map(|z| z as u8))
                .collect()
        })
        .collect();
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += step(&mut octopi);
    }

    println!("{}", flashes);
}
