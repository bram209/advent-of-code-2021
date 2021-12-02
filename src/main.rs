#![feature(array_windows)]

use std::{env, fs::read_to_string};

pub mod day1;
pub mod day2;

pub mod test_utils;

macro_rules! solvers {
    ($($day:literal,)*) => {
        paste::paste! {
            fn solve(day: usize) -> anyhow::Result<()> {
                let input = read_to_string(format!("src/day{}_input.txt", day))?;
                let (part1, part2) = match day {
                    $(
                    $day => (
                        [<day $day>]::part1(&[<day $day>]::parse(&input)?),
                        [<day $day>]::part2(&[<day $day>]::parse(&input)?)
                    ),
                    )*
                    _ => panic!("unsupported day: {}", day),
                };

                println!("Day {}:\n\tPart 1: {}\n\tPart 2: {}", day, part1, part2);
                Ok(())
            }
        }
    };
}

solvers! {
    1, 2,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Some(day) = args.get(1) {
        solve(day.parse().expect("day needs to be a valid number"))
            .expect("should solve without error");

        return;
    }

    panic!("please pass the day as argument")
}
