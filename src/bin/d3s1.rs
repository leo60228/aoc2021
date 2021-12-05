use aoc2021::prelude::*;

fn main() {
    let strings: Vec<String> = parsed_lines();
    let width = strings.iter().map(|x| x.len()).max().unwrap_or(0);
    let numbers: Vec<_> = strings
        .iter()
        .map(|x| usize::from_str_radix(x, 2).unwrap())
        .collect();
    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;

    for bit in (0..width).rev() {
        let counts = numbers.iter().map(|x| (x >> bit) & 1).counts();
        let most_common = counts
            .iter()
            .max_by_key(|&(_k, &v)| v)
            .map(|(&k, _v)| k)
            .unwrap_or(0);
        let least_common = counts
            .iter()
            .min_by_key(|&(_k, &v)| v)
            .map(|(&k, _v)| k)
            .unwrap_or(0);

        gamma_rate <<= 1;
        gamma_rate |= most_common;

        epsilon_rate <<= 1;
        epsilon_rate |= least_common;
    }

    println!("{}", gamma_rate * epsilon_rate);
}
