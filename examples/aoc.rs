//! Example implementation of an AoC solution.

use aoc_ornaments::{Part, Solution};

struct Day;

impl Solution for Day {
    type Output = String;
}

impl TryFrom<&str> for Day {
    type Error = miette::Error;

    fn try_from(_input: &str) -> miette::Result<Self> {
        Ok(Self)
    }
}

fn main() -> miette::Result<()>{
    let mut day = Day::try_from("aoc")?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}