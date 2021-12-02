use std::{num::ParseIntError, process::Output, usize};

use itertools::Itertools;

use crate::test_utils::tests;

pub fn parse(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input.lines().map(str::parse).collect()
}

pub fn part1(input: &[usize]) -> usize {
    input.array_windows().filter(|&[a, b]| b > a).count()
}

pub fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

tests! {

    lines_with_numbers:
        parse("123\n99\n200") equals Ok(vec![123, 99, 200]),

    lines_with_some_non_numeric:
        parse("123\nabc\n200") matches Err(..),

    all_increasing:
        part1(&[1, 2, 3]) equals 2,

    some_increasing:
        part1(&[1, 3, 3, 2, 4]) equals 2,

    none_increasing:
        part1(&[3, 2, 1]) equals 0,

    simple_example:
       part1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]) equals 7,

    simple_example:
       part2(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]) equals 5,

}
