//! Day 24: It Hangs in the Balance

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;

struct Day(Vec<usize>);

impl std::ops::Deref for Day {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let packages = input.lines().map(|line| line.parse().unwrap()).collect();

        Ok(Self(packages))
    }
}

impl Day {
    fn quantum_entanglement(&self, compartments: usize) -> Result<usize, miette::Error> {
        let total_weight = self.iter().sum::<usize>();
        let group_weight = total_weight / compartments;

        // Find first group of valid combinations
        let mut good_combinations = Vec::new();
        for n in 1..self.len() {
            good_combinations = self.iter()
                .combinations(n)
                .filter(|combo| combo.iter().copied().sum::<usize>() == group_weight)
                .map(|combo| combo.iter().copied().cloned().collect::<Vec<_>>())
                .collect();
                
            if !good_combinations.is_empty() {
                break;
            }
        }

        // Find the combination with smallest product
        if let Some(smallest) = good_combinations.iter()
            .min_by_key(|combo| combo.iter().product::<usize>()) 
        {
            let product: usize = smallest.iter().product();

            return Ok(product)
        }

        Err(miette::miette!("No valid combinations found"))
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.quantum_entanglement(3)
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.quantum_entanglement(4)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-24.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}