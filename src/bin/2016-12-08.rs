//! # Day 8: Two-Factor Authentication
//!

use std::str::FromStr;

use aoc_ornaments::{spatial::Position, ArgSolution, Part};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, i32, space0},
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
enum Dimension {
    X,
    Y,
}

#[derive(Debug)]
enum Operation {
    Rect(Position),
    Shift(Dimension, usize),
}

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<Operation>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        input.lines().for_each(|line| {
            Self::parse_line(line).unwrap();
        });

        todo!();
        // Ok(Self)
    }
}

impl Day {
    fn parse_line(input: &str) -> IResult<&str, Operation> {
        alt((Self::parse_rect, Self::parse_shift))(input)
    }

    fn parse_rect(input: &str) -> IResult<&str, Operation> {
        let (_, (_, x, _, y)) =
            preceded(tag("rect "), tuple((space0, i32, char('x'), i32)))(input)?;

        Ok(("", Operation::Rect(Position::new(x, y))))
    }

    fn parse_shift(input: &str) -> IResult<&str, Operation> {
        let (_, stuff) = preceded(
            tag("rotate "),
            tuple((
                space0,
                alt((tag("row y="), tag("column x="))),
                i32,
                space0,
                tag("by"),
                space0,
                i32,
            )),
        )(input)?;

        dbg!(&stuff);

        Ok(("", Operation::Shift(Dimension::X, 0)))
    }
}

impl ArgSolution<Position> for Day {
    type Output = usize;
}

fn main() -> miette::Result<()> {
    let mut puzzle = Day::from_str(include_str!("../inputs/2016-12-08.txt"))?;
    let part1 = puzzle.solve(Part::One, Position::new(50, 6))?;
    // let part2 = puzzle.solve(Part::Two)?;

    println!("Part 1: {part1}");
    // println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> miette::Result<()> {
        let mut day = Day::from_str(
            "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1",
        )?;

        assert_eq!(day.solve(Part::One, Position::new(7, 3))?, "6");

        Ok(())
    }

    #[test]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}
