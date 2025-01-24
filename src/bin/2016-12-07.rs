//! # Day 7: Internet Protocol Version 7
//!

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{
    branch::alt,
    character::complete::{alpha1, char},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<String>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Day(input
            .lines()
            // .map(String::from)
            .map(|line| {
                // let out = Self::parse_line(line).unwrap();

                // dbg!(&out);

                // let (input, (sequences, hypernet)) = tuple((
                //     separated_list1(delimited(char('['), alpha1, char(']')), alpha1),
                //     many0(delimited(char('['), alpha1, char(']'))),
                // ))(input)
                // .unwrap();
                // dbg!(input, sequences, hypernet);

                dbg!(Self::parse_line(line).unwrap());

                "".to_string()
            })
            .collect::<Vec<String>>()))
    }
}

impl Day {
    // sequences, hypernet sequences
    // fn parse_line(input: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    //     // let (a, b) = alt((alpha1, delimited(char('['), alpha1, char('['))))(input)?;
    //     let (a, b) = many1(alt((alpha1, delimited(char('['), alpha1, char('[')))))(input)?;

    //     dbg!(a, b);

    //     todo!()
    // }
    fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
        // many1(alt((alpha1, delimited(char('['), alpha1, char('[')))))(input)
        tuple((
            separated_list1(delimited(char('['), alpha1, char(']')), alpha1),
            many0(delimited(char('['), alpha1, char(']'))),
        ))(input)
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        // dbg!(&self);
        todo!()
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-07.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}
