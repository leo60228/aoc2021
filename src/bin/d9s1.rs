use aoc2021::prelude::*;

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

    let mut total_danger = 0;

    for (y, row) in floor.iter().enumerate() {
        for (x, &height) in row.iter().enumerate() {
            let neighbors = [
                floor.get(y.overflowing_sub(1).0).and_then(|row| row.get(x)),
                floor.get(y + 1).and_then(|row| row.get(x)),
                floor.get(y).and_then(|row| row.get(x.overflowing_sub(1).0)),
                floor.get(y).and_then(|row| row.get(x + 1)),
            ];
            let is_low = neighbors.into_iter().flatten().all(|&x| height < x);
            if is_low {
                let danger = height + 1;
                total_danger += danger;
            }
        }
    }

    println!("{}", total_danger);
}
