use aoc2021::prelude::*;
use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Instruction {
    X(usize),
    Y(usize),
}

fn fold(points: &mut HashSet<(usize, usize)>, instruction: Instruction) {
    let old = std::mem::take(points);
    *points = old
        .into_iter()
        .map(|(mut x, mut y)| {
            match instruction {
                Instruction::X(offset) => {
                    if x > offset {
                        x = offset - (x - offset);
                    }
                }
                Instruction::Y(offset) => {
                    if y > offset {
                        y = offset - (y - offset);
                    }
                }
            }

            (x, y)
        })
        .collect();
}

fn main() {
    let mut lines = parsed_lines::<String>().into_iter();
    let mut points = HashSet::new();
    let mut instructions = Vec::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (x, y) = line.split_once(',').unwrap();
        points.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    for line in lines {
        let instr = line.rsplit_once(' ').unwrap().1;
        let (ty, pos) = instr.split_once('=').unwrap();
        let pos = pos.parse().unwrap();

        instructions.push(match ty {
            "x" => Instruction::X(pos),
            "y" => Instruction::Y(pos),
            _ => unreachable!(),
        });
    }

    fold(&mut points, instructions[0]);

    println!("{}", points.len());
}
