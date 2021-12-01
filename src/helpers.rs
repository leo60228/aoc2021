use std::io::{prelude::*, stdin};
use std::str::FromStr;

pub fn parsed_lines<T: FromStr>() -> Vec<T> {
    stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|x| x.parse())
        .collect()
}
