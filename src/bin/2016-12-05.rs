//! # Day 5: How About a Nice Game of Chess?
//!

use std::str::FromStr;

use aoc_ornaments::{hash::AocHash, Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(AocHash);

impl FromStr for Day {
    type Err = miette::Error;

    /// .trim() because otherwise the /n will be included in the hash calculation (－‸ლ)
    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.trim().parse()?))
    }
}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<String> {
        let mut password = String::with_capacity(8);
        let mut idx = 0;

        while password.len() < 8 {
            let hash = AocHash::generate_hash_bytes(&self.0, idx);

            if AocHash::has_five_leading_zeros(&hash) {
                password.push(format!("{:x}", hash[2] & 0x0f).chars().next().unwrap());
            }

            idx += 1;
        }

        Ok(password)
    }
}

fn main() -> miette::Result<()> {
    let mut puzzle = Day::from_str(include_str!("../inputs/2016-12-05.txt"))?;

    println!(
        "Input bytes: {:?}",
        include_str!("../inputs/2016-12-05.txt").as_bytes()
    );
    println!("Length: {}", include_str!("../inputs/2016-12-05.txt").len());

    let part1 = puzzle.solve(Part::One)?;
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
        let mut day = Day::from_str("abc")?;
        assert_eq!(day.solve(Part::One)?, "18f47a30");
        Ok(())
    }

    #[test]
    fn part_2() -> miette::Result<()> {
        Ok(())
    }
}
