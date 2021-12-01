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
