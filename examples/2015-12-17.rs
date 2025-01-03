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

impl Day {
    fn compute() {
        todo!();
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok((1..=self.len())
            .flat_map(|container_size| {
                self.iter().combinations(container_size)
                .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
            }).count())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // let base = (1..=self.len())
        //     .flat_map(|container_size| {
        //         self.iter().combinations(container_size)
        //         .filter_map(|combo| {
        //             if combo.iter().copied().sum::<u32>() == 150 {
        //                 Some(combo.len())
        //             } else {
        //                 None
        //             }
        //         })
        //     })
        //     .inspect(|combo| {
        //         dbg!(combo);
        //     })
        //     .min().expect("No valid combinations found");

        let solutions: Vec<_> = (1..=self.len())
            .map(|size| {
                // For each size, get count of valid combinations
                self.iter()
                    .combinations(size)
                    .filter(|combo| combo.iter().copied().sum::<u32>() == 150)
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
    let mut day = Day::from_str(include_str!("./inputs/2015-12-17.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

