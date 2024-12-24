//! Example implementation of an AoC solution.

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

struct Day;

impl Solution for Day {
    type Output = String;
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

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}