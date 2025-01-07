//! Day 25: Let It Snow

use std::str::FromStr;

use aoc_ornaments::{spatial::{Position, UniquePositions}, Part, Solution};
use nom::{branch::alt, bytes::complete::{tag, take_until}, character::complete::{alpha1, digit1, space0}, combinator::{map, map_res}, sequence::{preceded, tuple}, IResult};

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
        // let (input, _) = take_until("row")(input).or_else(|_| take_until("column")(input)).map_err(|_| miette::miette!("bad"))?;
        // let (a, b) = tuple((alpha1, tag("row"), space0, digit1))(input).map_err(|e| miette::miette!("bad"))?;


        let (_, (x, y)) = Self::find_position(input).map_err(|e| miette::miette!(e.to_owned()))?;
        Ok(Self(Position::new(x, y)))
    }
}

impl Day {
    fn generate_codes() {
        todo!()
    }

    /// (x, y)
    fn find_position(input: &str) -> IResult<&str, (i32, i32)> {
        // let (input, _) = take_until(alt((tag("row"), tag("column"))))(input)?;
        let (input, _) = alt((take_until("row"), take_until("column")))(input)?;

        alt((
            // // row then column
            // tuple((
            //     number_after("row "),
            //     preceded(tag(" "), Self::number_after("column "))
            // )),
            // row then column - swap the tuple
            map(
                tuple((
                    Self::number_after("row "),
                    // preceded(tag(" "), Self::number_after("column "))
                    preceded(
                        tuple((space0, tag(","), space0)),
                        Self::number_after("column ")
                    ),
                )),
                |(row, col)| (col, row)
            ),
            // column then row
            tuple((
                Self::number_after("column "),
                // preceded(tag(" "), Self::number_after("row "))
                preceded(
                    tuple((space0, tag(","), space0)),
                    Self::number_after("row ")
                )
            ))
        ))(input)
    }

    fn number_after(prefix: &'static str) -> impl Fn(&str) -> IResult<&str, i32> {
        move |input| {
            map_res(
                preceded(tag(prefix), digit1),
                |s: &str| s.parse::<i32>()
            )(input)
        }
    }

    // fn find_num(input: &str) -> IResult<&str, usize> {
    //     todo!()
    // }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        dbg!(self);

        todo!()
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
