//! # Day 10: Balance Bots
//!

use std::{fmt::Display, str::FromStr};

use aoc_ornaments::{ArgSolution, Part};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::u32,
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug, derive_more::Deref)]
// struct Day(LogicCircuit<u32, Operation>);
struct Day(Vec<Operation>);

#[derive(Debug)]
enum Out {
    B(u32),
    O(u32),
}

impl Display for Out {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Out::B(x) => write!(f, "bot {x}"),
            Out::O(x) => write!(f, "output {x}"),
        }
    }
}

/// low, high
#[derive(Debug)]
struct Bot(Option<u32>, Option<u32>);

/// id, value
#[derive(Debug)]
struct Output(u32, u32);

#[derive(Debug)]
enum Operation {
    /// from bot, low to bot, high to bot
    Give(u32, Out, Out),
    /// bot, value
    Take(u32, u32),
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Give(from_bot, low, high) => {
                writeln!(f, "bot {from_bot} gives low to {low} and high to {high}")
            }
            Operation::Take(bot, value) => writeln!(f, "value {value} goes to bot {bot}"),
        }
    }
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

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().for_each(|i| write!(f, "{i}").unwrap());

        writeln!(f)
    }
}

impl Day {
    fn parse_line(input: &str) -> IResult<&str, Operation> {
        alt((Self::parse_bot, Self::parse_value))(input)
    }

    /// bot X gives low to bot|output Y and high to bot|output Z
    fn parse_bot(input: &str) -> IResult<&str, Operation> {
        let (input, from_bot) = preceded(tag("bot "), u32)(input)?;
        let (input, low) = Self::find_bot_or_output(input)?;
        let (_, high) = Self::find_bot_or_output(input)?;

        Ok(("", Operation::Give(from_bot, low, high)))
    }

    fn find_bot(input: &str) -> IResult<&str, &str> {
        take_until("bot")(input)
    }

    /// don't try: `take_until(alt((tag("bot"), tag("output"))))(input)``
    fn find_bot_or_output(input: &str) -> IResult<&str, Out> {
        alt((
            map(tuple((take_until("output"), Self::output_id)), |(_, id)| {
                Out::O(id)
            }),
            map(tuple((take_until("bot"), Self::bot_id)), |(_, id)| {
                Out::B(id)
            }),
        ))(input)
    }

    fn bot_id(input: &str) -> IResult<&str, u32> {
        preceded(tag("bot "), u32)(input)
    }

    fn output_id(input: &str) -> IResult<&str, u32> {
        preceded(tag("output "), u32)(input)
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
        // dbg!(&self);
        println!("{self}");

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
