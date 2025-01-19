//! Example implementation of a 3-part Everybody Codes solution.

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

struct Day;

impl Solution for Day {
    type Output = String;

    fn part3(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        todo!()
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(_input: &str) -> miette::Result<Self> {
        Ok(Self)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str("aoc")?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;
    let part3 = day.solve(Part::Three)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 3: {}", part3);

    Ok(())
}