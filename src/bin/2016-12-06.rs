//! # Day 6: Signals and Noise
//!

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day([Vec<char>; 8]);
// struct Day(HashMap<char, usize>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let cols = [const { Vec::<char>::new() }; 8];

        Ok(Self(input.lines().fold(cols, |mut acc, line| {
            line.chars().enumerate().for_each(|(i, x)| acc[i].push(x));

            acc
        })))
        // dbg!(Self(input.lines().scan(cols, |c, &line| {
        //     line.chars().enumerate().for_each(|(i, x)| c[i].push(x));

        //     c
        // })));

        // Ok(Self(input.lines().fold(HashMap::new(), |mut acc, line| {
        //     line.chars().for_each(|c| {
        //         *acc.entry(c).or_insert(0) += 1;
        //     });
        //     acc
        // })))
    }
}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        // dbg!(&self);

        for chars in self.iter() {
            let mut map = HashMap::new();
            // dbg!(&chars);
            chars.iter().for_each(|k| {
                *map.entry(k).or_insert(0) += 1;
            });

            // dbg!(&map);
            dbg!(map.iter().max_by_key(|(_, count)| *count).unwrap().0);
        }

        Ok("".to_string())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        // dbg!(&self);

        for chars in self.iter() {
            let mut map = HashMap::new();
            // dbg!(&chars);
            chars.iter().for_each(|k| {
                *map.entry(k).or_insert(0) += 1;
            });

            dbg!(map.iter().min_by_key(|(_, count)| *count).unwrap().0);
        }

        todo!()
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
