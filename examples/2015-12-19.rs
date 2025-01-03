//! Day 19: Medicine for Rudolph

use std::{collections::HashMap, str::FromStr};

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
    fn from(replacements: (Vec<(&str, &str, &str)>, &str)) -> Self {
        Self::from((replacements.0.iter().map(|(from, _, to)| {
            format!("{} => {}", from, to)
        }).collect::<Vec<_>>(), replacements.1.to_string()))
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (target_molecule, molecules) = Self::parse_molecules(input).expect("invalid input");
        let (_, target_molecule) = preceded(
            multispace0::<&str, nom::error::Error<&str>>,
            take_while1(|c: char| c.is_ascii_alphabetic()),
        )(target_molecule).expect("ok");

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

    fn count_steps(&self) -> usize {
        let molecule = &self.1;
        
        // Count specific patterns
        let rn_count = molecule.matches("Rn").count();
        let ar_count = molecule.matches("Ar").count();
        let y_count = molecule.matches('Y').count();
        
        // Count total atoms (each uppercase + following lowercase is one atom)
        let atom_count = molecule.chars()
            .enumerate()
            .filter(|(i, c)| {
                if !c.is_ascii_uppercase() {
                    return false;
                }
                // Include this uppercase letter as an atom start
                true
            })
            .count();
            
        // println!("Molecule: {}", molecule);
        // println!("Atoms: {}", atom_count);
        // println!("Rn: {}", rn_count);
        // println!("Ar: {}", ar_count);
        // println!("Y: {}", y_count);
            
        // The formula should be:
        atom_count - rn_count - ar_count - 2 * y_count - 1
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.generate_molecules().len())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.count_steps())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-19.txt"))?;
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
    #[case("HOH", 4)]
    #[case("HOHOHO", 7)]
    fn test_part1(#[case] input: &str, #[case] expected: usize) {
        let input = "H => HO
H => OH
O => HH";
        // let mut day = Day::from_str(input).unwrap();
        // let result = day.solve(Part::One).unwrap();

        // assert_eq!(result, expected.to_string());

        todo!()
    }

    #[rstest]
    #[case("HOH", 3)]
    #[case("HOHOHO", 6)]
    fn test_part2(#[case] input: &str, #[case] expected: usize) {
        let input = "e => H
e => O
H => HO
H => OH
O => HH";
        // let mut day = Day::from_str(input).unwrap();
        // let result = day.solve(Part::Two).unwrap();

        // assert_eq!(result, expected.to_string());

        todo!()
    }
}
