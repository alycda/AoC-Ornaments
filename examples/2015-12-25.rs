//! Day 25: Let It Snow

use std::str::FromStr;

use aoc_ornaments::{spatial::{Position, UniquePositions}, Part, Solution};
use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{digit1, space0}, combinator::{map, map_res}, sequence::{preceded, tuple}, IResult};

#[derive(Debug)]
struct Day(Position);

impl std::ops::Deref for Day {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, (x, y)) = Self::find_position(input)
            .map_err(|e| miette::miette!(e.to_owned()))?;

        Ok(Self(Position::new(x, y)))
    }
}

impl Day {
    fn generate_codes(first: usize) -> impl Iterator<Item = (Position, usize)> {
        std::iter::successors(Some((Position::ONE, first)), |&(pos, prev)| {
            let north_east = pos + Position::new(1, -1);
            let down = Position::new(1, pos.x + 1);
            
            // dbg!(&north_east, &down);

            let next = if north_east.y < 1 {
                down
            } else {
                north_east
            };

            Some((next, (prev * 252533) % 33554393))
        })
    }

    /// (x, y)
    fn find_position(input: &str) -> IResult<&str, (i32, i32)> {
        let (input, _) = alt((take_until("row"), take_until("column")))(input)?;

        Self::find_coordinates(input)
    }

    fn prefixed_number(prefix: &'static str) -> impl Fn(&str) -> IResult<&str, i32> {
        move |input| {
            map_res(
                preceded(tag(prefix), digit1),
                |s: &str| s.parse::<i32>()
            )(input)
        }
    }

    fn find_coordinates(input: &str) -> IResult<&str, (i32, i32)> {
        alt((
            // row then column - swap the tuple
            map(
                tuple((
                    Self::prefixed_number("row "),
                    preceded(
                        tuple((space0, tag(","), space0)),
                        Self::prefixed_number("column ")
                    ),
                )),
                |(row, col)| (col, row)
            ),
            // column then row
            tuple((
                Self::prefixed_number("column "),
                preceded(
                    tuple((space0, tag(","), space0)),
                    Self::prefixed_number("row ")
                )
            ))
        ))(input)
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let mut generator = Self::generate_codes(20151125);

        generator
            .find(|(pos, _)| *pos == **self)
            .map(|(_, code)| code)
            .ok_or_else(|| miette::miette!("No code found"))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-25.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}
