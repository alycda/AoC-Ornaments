//! # Day 6: Signals and Noise
//!

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<HashMap<char, usize>>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let cols: Vec<_> = (0..8).map(|_| HashMap::new()).collect();

        Ok(Self(input.lines().fold(cols, |mut c, line| {
            line.chars()
                .enumerate()
                .for_each(|(i, x)| *c[i].entry(x).or_insert(0) += 1);

            c
        })))
    }
}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self
            .iter()
            .map(|m| m.iter().max_by_key(|(_, count)| *count).unwrap().0)
            .collect())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self
            .iter()
            .map(|m| m.iter().min_by_key(|(_, count)| *count).unwrap().0)
            .collect())
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-06.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
