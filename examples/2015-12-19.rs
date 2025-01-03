//! Day 19: Medicine for Rudolph

use std::{collections::{HashMap, HashSet}, str::FromStr};

use aoc_ornaments::{linear::Uniqued, Part, Solution};
use nom::{bytes::complete::{tag, take_while1}, character::complete::{alpha1, multispace0}, multi::separated_list1, sequence::{preceded, tuple}, IResult};

type Molecule = String;
type Replacements = Vec<String>;

#[derive(Debug)]
struct Day(HashMap<Molecule, Replacements>, Molecule);

impl std::ops::Deref for Day {
    // type Target = Uniqued;
    type Target = HashMap<Molecule, Replacements>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<(Replacements, Molecule)> for Day {
    fn from(replacements: (Replacements, Molecule)) -> Self {
        let mut map = HashMap::new();

        replacements.0.iter().for_each(|replacement| {
            let parts: Vec<&str> = replacement.split_whitespace().collect();

            match parts.as_slice() {
                [from, "=>", to] => {
                    map.entry(from.to_string())
                        .or_insert_with(Vec::new)
                        .push(to.to_string());
                }
                _ => panic!("Invalid input"),
            }
        });

        Self(map, replacements.1)
    }
}

impl From<(Vec<(&str, &str, &str)>, &str)> for Day {
    // fn from((from, _, to): (&str, &str, &str)) -> Self {
    //     let replacements = vec![from.to_string(), to.to_string()];
    //     Self::from(replacements)
    // }

    fn from(replacements: (Vec<(&str, &str, &str)>, &str)) -> Self {
        // let replacements = 
        Self::from((replacements.0.iter().map(|(from, _, to)| {
            format!("{} => {}", from, to)
        }).collect::<Vec<_>>(), replacements.1.to_string()))

        // Self::from(replacements)
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (target_molecule, molecules) = Self::parse_molecules(input).expect("invalid input");
        // let (target_molecule, _): (&str, &str) = preceded(tag("\n\n"), alpha1)(input).expect("invalid input");
        let (_, target_molecule) = preceded(
            multispace0::<&str, nom::error::Error<&str>>,
            take_while1(|c: char| c.is_ascii_alphabetic()),
        )(target_molecule).expect("ok");

        // dbg!(&target_molecule, &molecules);

        // todo!();
        Ok(Self::from((molecules, target_molecule)))
    }
}

impl Day {
    fn parse_molecules(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
        separated_list1(tag("\n"), Self::parse_molecule)(input)
    }

    fn parse_molecule(input: &str) -> IResult<&str, (&str, &str, &str)> {
        tuple((alpha1, tag(" => "), alpha1))(input)
    }

    fn unique_molecules(&self) -> Uniqued {
        let mut molecules = Uniqued::new();

        for (from, to) in self.iter() {
            for replacement in to {
                molecules.insert(replacement.clone());
            }
        }

        dbg!(molecules)
    }

    fn generate_molecules(&self) -> Uniqued {
        let mut results = Uniqued::new();
        
        // For each (from, to_list) in our replacements
        for (from, to_list) in self.iter() {
            // Find all positions where 'from' occurs in our target molecule
            let mut start = 0;
            while let Some(pos) = self.1[start..].find(from) {
                let actual_pos = start + pos;
                
                // For each possible replacement
                for to in to_list {
                    // Create new molecule by replacing at this position
                    let new_molecule = format!(
                        "{}{}{}",
                        &self.1[..actual_pos],
                        to,
                        &self.1[actual_pos + from.len()..]
                    );
                    results.insert(new_molecule);
                }
                
                start = actual_pos + 1;
            }
        }
        
        results
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // dbg!(&self);

        // todo!()

        Ok(self.generate_molecules().len())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-19.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("HOH", 4)]
    #[case("HOHOHO", 7)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        // let mut day = Day::from_str(input).unwrap();
        // let result = day.solve(Part::One).unwrap();

        // assert_eq!(result, expected.to_string());

        todo!()
    }
}
