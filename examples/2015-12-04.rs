//! Day 4: The Ideal Stocking Stuffer

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug)]
struct Day(String);

impl std::ops::Deref for Day {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.to_owned()))
    }
}

impl Day {
    // fn generate_hash(key: &str, number: u32) -> String {
    //     let input = format!("{}{}", key, number);
    //     format!("{:x}", md5::compute(input))
    // }

    fn generate_hash_bytes(key: &str, number: u32) -> [u8; 16] {
        let input = format!("{}{}", key, number);
        md5::compute(input).0
    }
    
    fn has_five_leading_zeros(hash: &[u8; 16]) -> bool {
        // First two bytes must be 0
        hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xf0) == 0
    }
}

impl Solution for Day {
    type Output = u32;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // dbg!(&self);

        // dbg!(Day::generate_hash(&self.0, 0));
        // dbg!(Day::generate_hash_bytes(&self.0, 0));

        (1..).find(|&n| {
            Day::has_five_leading_zeros(&Day::generate_hash_bytes(&self.0, n))
        }).ok_or_else(|| miette::miette!("No solution found"))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-04.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}