//! # Day 10: Balance Bots
//!

use std::str::FromStr;

use aoc_ornaments::{ArgSolution, Part};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::u32,
    sequence::preceded,
    IResult,
};

#[derive(Debug, derive_more::Deref)]
// struct Day(LogicCircuit<u32, Operation>);
struct Day(Vec<Operation>);

/// low, high
#[derive(Debug)]
struct Bot(Option<u32>, Option<u32>);

#[derive(Debug)]
enum Operation {
    /// from bot, low to bot, high to bot
    Give(u32, u32, u32),
    /// bot, value
    Take(u32, u32),
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| Self::parse_line(line).unwrap().1)
                .collect::<Vec<_>>(),
        ))
    }
}

impl Day {
    fn parse_line(input: &str) -> IResult<&str, Operation> {
        alt((Self::parse_bot, Self::parse_value))(input)
    }

    /// bot X gives low to bot|output Y and high to bot|output Z
    fn parse_bot(input: &str) -> IResult<&str, Operation> {
        let (input, from_bot) = preceded(tag("bot "), u32)(input)?;
        let (input, _) = Self::find_bot(input)?;
        let (input, low_bot) = Self::bot_id(input)?;
        let (input, _) = Self::find_bot(input)?;
        let (_, high_bot) = Self::bot_id(input)?;

        Ok(("", Operation::Give(from_bot, low_bot, high_bot)))
    }

    fn find_bot(input: &str) -> IResult<&str, &str> {
        take_until("bot")(input)
    }

    // fn find_bot_or_output(input: &str) -> IResult<&str, &str> {
    //     take_until(alt((tag("bot"), tag("output"))))
    // }

    fn bot_id(input: &str) -> IResult<&str, u32> {
        preceded(tag("bot "), u32)(input)
    }

    /// value X goes to bot Y
    fn parse_value(input: &str) -> IResult<&str, Operation> {
        let (input, value) = preceded(tag("value "), u32)(input)?;
        let (input, _) = Self::find_bot(input)?;
        let (_, bot_id) = Self::bot_id(input)?;

        Ok(("", Operation::Take(bot_id, value)))
    }
}

impl ArgSolution<(u32, u32)> for Day {
    type Output = u32;

    fn part1(&mut self, _args: (u32, u32)) -> aoc_ornaments::SolutionResult<Self::Output> {
        dbg!(&self);

        todo!()
    }
}

fn main() -> miette::Result<()> {
    let mut puzzle = Day::from_str(include_str!("../inputs/2016-12-10.txt"))?;
    let part1 = puzzle.solve(Part::One, (61, 17))?;
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
        let input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";
        let mut day = Day::from_str(input)?;
        assert_eq!(day.solve(Part::One, (5, 2))?, "2");

        Ok(())
    }

    #[test]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}
