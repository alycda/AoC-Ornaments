//! Day 4: The Ideal Stocking Stuffer

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(String);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.to_owned()))
    }
}

impl Day {
    fn generate_hash_bytes(key: &str, number: u32) -> [u8; 16] {
        let input = format!("{}{}", key, number);
        md5::compute(input).0
    }
    
    /// First two bytes must be 0
    fn has_five_leading_zeros(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xf0) == 0
    }
    
    /// First three bytes must be 0
    fn has_six_leading_zeros(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    }

    fn compute(&self, l: fn(&[u8; 16]) -> bool) -> Option<u32> {
        (1..).find(|&n| l(&Day::generate_hash_bytes(&self.0, n)))
    }
}

impl Solution for Day {
    type Output = u32;

    /// Find the lowest positive number that, when combined with the input string, produces an MD5 hash with five leading zeros.
    fn part1(&mut self) -> miette::Result<Self::Output> {
        self.compute(Self::has_five_leading_zeros).ok_or_else(|| miette::miette!("No solution found"))
    }

    /// Find the lowest positive number that, when combined with the input string, produces an MD5 hash with six leading zeros.
    fn part2(&mut self) -> miette::Result<Self::Output> {
        self.compute(Self::has_six_leading_zeros).ok_or_else(|| miette::miette!("No solution found"))
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