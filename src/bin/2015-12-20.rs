//! Day 20: Infinite Elves and Infinite Houses

use std::str::FromStr;

use aoc_ornaments::{Part, ArgSolution};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, Clone, Copy)]
struct Day(usize);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.trim().parse().expect("valid number")))
    }
}

impl ArgSolution<usize> for Day {
    type Output = usize;

    fn part1(&mut self, count: usize) -> aoc_ornaments::SolutionResult<Self::Output> {
        let target = **self;
        let size = target / count;
        let mut houses = vec![0; size];
        
        // Each elf visits their multiples
        for elf in 1..size {
            // Visit each multiple of elf number
            for house in (elf..size).step_by(elf) {
                houses[house] += elf * count;
            }
            
            // Check if this house now has enough
            if houses[elf] >= target {
                return Ok(elf);
            }
        };

        miette::bail!("No house found");
    }

    fn part2(&mut self, count: usize) -> aoc_ornaments::SolutionResult<Self::Output> {
        let target = **self;
        let size = target / count;
        let mut houses = vec![0; size];
        
        // Each elf visits their multiples
        for elf in 1..size {
            // Visit only first 50 multiples of this elf
            let mut visits = 0;
            for house in (elf..size).step_by(elf) {
                houses[house] += elf * count;
                visits += 1;
                if visits >= 50 {
                    break;
                }
            }
            
            // Check if this house now has enough
            if houses[elf] >= target {
                return Ok(elf);
            }
        }

        miette::bail!("No house found");
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-20.txt"))?;
    let part1 = day.solve(Part::One, 10)?;
    let part2 = day.solve(Part::Two, 11)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}