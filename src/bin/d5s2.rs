use aoc2021::prelude::*;
use std::cmp::{max, min};
use std::str::FromStr;

pub struct Line {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(x: &str) -> Result<Self, ()> {
        let (start, end) = x.split_once(" -> ").ok_or(())?;
        let (x1, y1) = start.split_once(',').ok_or(())?;
        let (x2, y2) = end.split_once(',').ok_or(())?;

        Ok(Self {
            x1: x1.parse().map_err(drop)?,
            y1: y1.parse().map_err(drop)?,
            x2: x2.parse().map_err(drop)?,
            y2: y2.parse().map_err(drop)?,
        })
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let lines = parsed_lines::<Line>();

    let width = lines.iter().flat_map(|x| [x.x1, x.x2]).max().unwrap_or(0) + 1;
    let height = lines.iter().flat_map(|x| [x.y1, x.y2]).max().unwrap_or(0) + 1;

    let mut map = vec![vec![0usize; width]; height];

    for line in lines {
        if line.x1 == line.x2 {
            let y_start = min(line.y1, line.y2);
            let y_end = max(line.y1, line.y2);
            for y in y_start..=y_end {
                map[y][line.x1] += 1;
            }
        } else if line.y1 == line.y2 {
            let x_start = min(line.x1, line.x2);
            let x_end = max(line.x1, line.x2);
            for x in x_start..=x_end {
                map[line.y1][x] += 1;
            }
        } else {
            let ((x1, y1), (x2, y2)) = if line.y1 > line.y2 {
                ((line.x2, line.y2), (line.x1, line.y1))
            } else {
                ((line.x1, line.y1), (line.x2, line.y2))
            };

            let mut x = x1;

            for y in y1..=y2 {
                map[y][x] += 1;

                if x1 < x2 {
                    x += 1;
                } else {
                    x = x.overflowing_sub(1).0;
                }
            }
        }
    }

    let danger = map.iter().flatten().filter(|&&x| x >= 2).count();

    println!("{}", danger);
}
