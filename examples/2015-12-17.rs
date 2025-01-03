//! Day 17: No Such Thing as Too Much

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;

struct Day(Vec<u32>);

impl std::ops::Deref for Day {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().map(|line| line.parse().unwrap()).collect()))
    }
}

impl Day {}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok((1..=self.len())
            .flat_map(|container_size| {
                self.iter().combinations(container_size)
                .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
            }).count())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-17.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}

