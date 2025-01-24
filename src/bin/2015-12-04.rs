//! Day 4: The Ideal Stocking Stuffer

use std::str::FromStr;

use aoc_ornaments::{hash::AocHash, Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(AocHash);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Day {
    fn compute(&self, l: fn(&[u8; 16]) -> bool) -> Option<u32> {
        (1..).find(|&n| l(&AocHash::generate_hash_bytes(&self.0, n)))
    }
}

impl Solution for Day {
    type Output = u32;

    /// Find the lowest positive number that, when combined with the input string, produces an MD5 hash with five leading zeros.
    fn part1(&mut self) -> miette::Result<Self::Output> {
        self.compute(AocHash::has_five_leading_zeros).ok_or_else(|| miette::miette!("No solution found"))
    }

    /// Find the lowest positive number that, when combined with the input string, produces an MD5 hash with six leading zeros.
    fn part2(&mut self) -> miette::Result<Self::Output> {
        self.compute(AocHash::has_six_leading_zeros).ok_or_else(|| miette::miette!("No solution found"))
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-04.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abcdef", 609043)]
    #[case("pqrstuv", 1048970)]
    fn part_1(#[case] input: &str, #[case] expected: usize) {
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
    }
}