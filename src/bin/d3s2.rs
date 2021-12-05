use aoc2021::prelude::*;

enum Rating {
    Oxygen,
    Co2,
}

fn find_rating(mut numbers: Vec<usize>, width: usize, rating: Rating) -> Option<usize> {
    for bit in (0..width).rev() {
        let counts = numbers.iter().map(|x| (x >> bit) & 1).counts();
        let target = match rating {
            Rating::Oxygen => {
                if counts[&0] <= counts[&1] {
                    1
                } else {
                    0
                }
            }
            Rating::Co2 => {
                if counts[&0] <= counts[&1] {
                    0
                } else {
                    1
                }
            }
        };
        numbers.retain(|x| (x >> bit) & 1 == target);

        if numbers.len() == 1 {
            return Some(numbers[0]);
        }
    }

    None
}

fn main() {
    let strings: Vec<String> = parsed_lines();
    let width = strings.iter().map(|x| x.len()).max().unwrap_or(0);
    let numbers: Vec<_> = strings
        .iter()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();

    let oxygen = find_rating(numbers.clone(), width, Rating::Oxygen).unwrap();
    let co2 = find_rating(numbers, width, Rating::Co2).unwrap();

    println!("{}", oxygen * co2);
}
