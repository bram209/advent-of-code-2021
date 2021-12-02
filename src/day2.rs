use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

use crate::test_utils::tests;

use Movement::*;

#[derive(Debug, PartialEq)]
pub enum Movement {
    Forward(usize),
    Up(usize),
    Down(usize),
}

pub fn parse(input: &str) -> Result<Vec<Movement>> {
    input.lines().map(parse_movement).collect()
}

fn parse_movement(input: &str) -> Result<Movement> {
    if let Some((movement, amount)) = input.split(' ').collect_tuple() {
        let amount = amount.parse()?;

        let parsed = match movement {
            "forward" => Forward(amount),
            "down" => Down(amount),
            "up" => Up(amount),
            _ => bail!("unknown movement: {}", movement),
        };

        Ok(parsed)
    } else {
        Err(anyhow!("invalid input: {}", input))
    }
}

pub fn part1(movements: &[Movement]) -> usize {
    let (total_horizontal_pos, total_depth) =
        movements
            .iter()
            .fold((0, 0), |(horizontal_pos, depth), movement| match movement {
                Forward(amount) => (horizontal_pos + amount, depth),
                Up(amount) => (horizontal_pos, depth - amount),
                Down(amount) => (horizontal_pos, depth + amount),
            });

    return total_horizontal_pos * total_depth;
}

pub fn part2(movements: &[Movement]) -> usize {
    let (total_horizontal_pos, total_depth, _) = movements.iter().fold(
        (0, 0, 0),
        |(horizontal_pos, depth, aim), movement| match movement {
            Forward(amount) => (horizontal_pos + amount, depth + (amount * aim), aim),
            Up(amount) => (horizontal_pos, depth, aim - amount),
            Down(amount) => (horizontal_pos, depth, aim + amount),
        },
    );

    return total_horizontal_pos * total_depth;
}

tests! {

    valid_data:
        parse("forward 5\ndown 5\nup 8") ok_and_equals vec![Forward(5), Down(5), Up(8)],

    invalid_movement:
        parse("diagonal 5") matches Err(..),

    invalid_movement_amount:
        parse("forward a4") matches Err(..),

    only_down:
        part1(&vec![Down(2), Down(4)]) equals 0,

    down_and_forward:
        part1(&vec![Forward(1), Down(2), Forward(2), Down(4)]) equals 18,

    down_up_and_forward:
        part1(&vec![Forward(1), Down(8), Up(2), Forward(2), Up(4)]) equals 6,

    example_data:
        part1(&vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]) equals 150,

    example_data:
        part2(&vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]) equals 900,

}
