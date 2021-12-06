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

pub fn split_line<T: FromStr>(ch: char) -> Vec<T> {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).unwrap();
    line.trim().split(ch).flat_map(|x| x.parse()).collect()
}

pub trait Summable: Sized {
    type Target: std::iter::Sum<Self>;
}

impl Summable for usize {
    type Target = usize;
}

impl Summable for &usize {
    type Target = usize;
}

pub fn sum<I>(iter: I) -> <I::Item as Summable>::Target
where
    I: IntoIterator,
    I::Item: Summable,
{
    iter.into_iter().sum()
}
