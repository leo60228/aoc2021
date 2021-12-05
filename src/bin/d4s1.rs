use aoc2021::prelude::*;

fn board_wins(board: &[Vec<usize>], numbers: &[usize]) -> bool {
    let width = board[0].len();

    for row in board {
        if row.iter().all(|x| numbers.contains(x)) {
            return true;
        }
    }

    for column in 0..width {
        if board.iter().all(|row| numbers.contains(&row[column])) {
            return true;
        }
    }

    false
}

fn main() {
    let stdin = stdin();
    let mut lines = stdin.lock().lines().flatten();

    let all_numbers: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards = Vec::new();

    while lines.next().is_some() {
        let mut board = Vec::new();

        for _ in 0..5 {
            let line: Vec<usize> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap())
                .collect();
            board.push(line);
        }

        boards.push(board);
    }

    let mut numbers = Vec::new();

    for number in all_numbers {
        numbers.push(number);

        for board in &boards {
            if board_wins(&board[..], &numbers) {
                let unmarked: usize = board
                    .iter()
                    .flatten()
                    .filter(|x| !numbers.contains(x))
                    .sum();
                let score = unmarked * number;

                println!("{}", score);

                return;
            }
        }
    }
}
