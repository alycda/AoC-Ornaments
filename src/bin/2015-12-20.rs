//! Day 20: Infinite Elves and Infinite Houses

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut, Clone, Copy)]
struct Day(usize);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.trim().parse().expect("valid number")))
    }
}

impl Day { 
    fn _house_presents() {
        todo!()
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let target = **self;
        let size = target / 10;
        let mut houses = vec![0; size];
        
        // Each elf visits their multiples
        for elf in 1..size {
            // Visit each multiple of elf number
            for house in (elf..size).step_by(elf) {
                houses[house] += elf * 10;
            }
            
            // Check if this house now has enough
            if houses[elf] >= target {
                return Ok(elf);
            }
        };

        miette::bail!("No house found");
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let target = **self;
        let size = target / 11;
        let mut houses = vec![0; size];
        
        // Each elf visits their multiples
        for elf in 1..size {
            // Visit only first 50 multiples of this elf
            let mut visits = 0;
            for house in (elf..size).step_by(elf) {
                houses[house] += elf * 11;
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
    #[case(1, 10)]
    #[case(2, 30)]
    #[case(3, 40)]
    #[case(4, 70)]
    #[case(5, 60)]
    #[case(6, 120)]
    #[case(7, 80)]
    #[case(8, 150)]
    #[case(9, 130)]
    fn test_house_presents(#[case] house: usize, #[case] presents: usize) {
        todo!()
    }
}