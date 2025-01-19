//! Day 17: No Such Thing as Too Much

use std::str::FromStr;

use aoc_ornaments::{Part, ArgSolution};
use itertools::Itertools;

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<u32>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().map(|line| line.parse().unwrap()).collect()))
    }
}

impl ArgSolution<u32> for Day {
    type Output = usize;

    fn part1(&mut self, count: u32) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok((1..=self.len())
            .flat_map(|container_size| {
                self.iter()
                    .combinations(container_size)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == count)
            }).count())
    }

    fn part2(&mut self, count: u32) -> aoc_ornaments::SolutionResult<Self::Output> {
        let solutions: Vec<_> = (1..=self.len())
            .map(|size| {
                // For each size, get count of valid combinations
                self.iter()
                    .combinations(size)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == count)
                    .count()
            })
            .enumerate()  // Keep track of the size
            .filter(|&(_, count)| count > 0)  // Only keep sizes that have solutions
            .collect();

        // The first (minimum) size that has solutions
        if let Some(&(_, count)) = solutions.first() {
            Ok(count)
        } else {
            Err(miette::miette!("no combinations found"))
        }
        
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-17.txt"))?;
    let part1 = day.solve(Part::One, 150)?;
    let part2 = day.solve(Part::Two, 150)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut day = Day::from_str("20\n15\n10\n5\n5").unwrap();
        assert_eq!(day.solve(Part::One, 25).unwrap(), "4");
    }
}