//! Day 20: Infinite Elves and Infinite Houses

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug, Clone, Copy)]
struct Day(usize);

impl std::ops::Deref for Day {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Day {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.trim().parse().expect("valid number")))
    }
}

impl Day {
    fn find_first_house(target: usize) -> usize {
        // Make array big enough - we can estimate size needed
        let size = target / 10;  // rough estimate
        let mut houses = vec![0; size];
        
        // Each elf visits their multiples
        for elf in 1..size {
            // Visit each multiple of elf number
            for house in (elf..size).step_by(elf) {
                houses[house] += elf * 10;
            }
            
            // Check if this house now has enough
            if houses[elf] >= target {
                return elf;
            }
        }
        0
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // dbg!(&self);

        // ((self.0 / 40)..=self.0 / 10).into_iter().position(|house| {
        // // (1..=9).into_iter().position(|house| {
        //     // let presents = (1..=house).filter(|elf| house % elf == 0).sum::<usize>() * 10;
        //     // dbg!(presents) >= **self

        //     let mut sum = 0;
        //     let sqrt = (house as f64).sqrt() as usize;
        //     for elf in 1..=sqrt {
        //         if house % elf == 0 {
        //             sum += elf;
        //             if elf != house/elf {  // avoid counting sqrt twice for perfect squares
        //                 sum += house/elf;
        //             }
        //         }
        //     }
        //     (sum * 10) >= **self
        // }).ok_or(miette::miette!("no solution")).map(|pos| pos + 1)
        
        // // .position(|house| {
        // //     let presents = (1..=*house).filter(|elf| *house % elf == 0).sum::<usize>() * 10;
        // //     presents >= *self
        // // }).unwrap().to_owned().into()

        // // .find(|&house| {
        // //     let presents = (1..=*house).filter(|elf| *house % elf == 0).sum::<usize>() * 10;
        // //     presents >= *self
        // // }).unwrap().to_owned().into();

        // // todo!()

        Ok(Self::find_first_house(**self))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-20.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); // > 761
    // println!("Part 2: {}", part2);

    Ok(())
}