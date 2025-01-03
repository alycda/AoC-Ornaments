//! Day 19: Medicine for Rudolph

use std::{collections::{HashMap, HashSet, VecDeque}, str::FromStr};

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

    // fn reduce_molecule(&self) -> usize {
    //     let mut reverse_replacements: Vec<(String, String)> = self
    //         .iter()
    //         .flat_map(|(from, to_list)| {
    //             to_list.iter().map(|to| (to.clone(), from.clone()))
    //         })
    //         .collect();
        
    //     // Sort by length of 'to' (longest first)
    //     reverse_replacements.sort_by_key(|(to, _)| std::cmp::Reverse(to.len()));
        
    //     let mut visited = Vec::new();  // Track visited states
    //     let mut queue = vec![(self.1.clone(), 0)];  // (string, count)
    //     let mut solution = Vec::new();  // Track our path
    //     let mut stack = Vec::new();  // Backup candidates
        
    //     while let Some((string, count)) = queue.pop() {
    //         if string == "e" {
    //             return count;
    //         }
            
    //         // Find all candidates where their replacement appears in our string
    //         let candidates: Vec<_> = reverse_replacements.iter()
    //             .filter(|(to, _)| string.contains(to))
    //             .flat_map(|(to, from)| {
    //                 // For each occurrence, create a new candidate
    //                 string.match_indices(to)
    //                     .map(|(pos, _)| {
    //                         let next = format!(
    //                             "{}{}{}",
    //                             &string[..pos],
    //                             from,
    //                             &string[pos + to.len()..]
    //                         );
    //                         (next, count + 1)
    //                     })
    //             })
    //             .filter(|(next, _)| !visited.contains(next))
    //             .collect();
            
    //         // Take the first candidate, push rest to stack
    //         if let Some(next) = candidates.first().cloned() {
    //             stack.extend(candidates.into_iter().skip(1));
    //             visited.push(next.0.clone());
    //             queue.push(next);
    //             solution.push(next);
    //         } else {
    //             // Dead end - backtrack
    //             solution.pop();
    //             if let Some(backup) = stack.pop() {
    //                 visited.push(backup.0.clone());
    //                 queue.push(backup);
    //                 solution.push(backup);
    //             }
    //         }
    //     }
        
    //     panic!("No solution found!")
    // }
    

    // fn reduce_molecule(&self) -> usize {
    //     let mut reverse_replacements: Vec<(String, String)> = self
    //         .iter()
    //         .flat_map(|(from, to_list)| {
    //             to_list.iter().map(|to| (to.clone(), from.clone()))
    //         })
    //         .collect();
        
    //     // Sort by length of 'to' (longest first)
    //     reverse_replacements.sort_by_key(|(to, _)| std::cmp::Reverse(to.len()));
        
    //     let mut current = self.1.clone();
    //     let mut steps = 0;
    //     let mut visited = HashSet::new();
    //     let mut queue = VecDeque::new();
        
    //     // Initialize with starting state
    //     queue.push_back((current.clone(), steps));
    //     visited.insert(current.clone());
        
    //     while let Some((molecule, count)) = queue.pop_front() {
    //         if molecule == "e" {
    //             return count;
    //         }
            
    //         // Try each candidate replacement
    //         for (to, from) in &reverse_replacements {
    //             if let Some(pos) = molecule.find(to) {
    //                 let next = format!(
    //                     "{}{}{}",
    //                     &molecule[..pos],
    //                     from,
    //                     &molecule[pos + to.len()..]
    //                 );
                    
    //                 if !visited.contains(&next) {
    //                     visited.insert(next.clone());
    //                     queue.push_back((next, count + 1));
    //                     // Take just the first replacement for the greedy approach
    //                     break;
    //                 }
    //             }
    //         }
    //     }
        
    //     panic!("No solution found!")
    // }

    // fn reduce_molecule(&self) -> usize {
    //     // Create reverse mapping: to -> from
    //     let mut reverse_replacements: Vec<(String, String)> = self
    //         .iter()
    //         .flat_map(|(from, to_list)| {
    //             to_list.iter().map(|to| (to.clone(), from.clone()))
    //         })
    //         .collect();
        
    //     // Sort by length of 'to' (longest first)
    //     reverse_replacements.sort_by_key(|(to, _)| std::cmp::Reverse(to.len()));
        
    //     let mut current = self.1.clone();  // Start with target molecule
    //     let mut steps = 0;
        
    //     while current != "e" {
    //         let previous = current.clone();
            
    //         // Try each replacement
    //         for (to, from) in &reverse_replacements {
    //             if let Some(pos) = current.find(to) {
    //                 current = format!(
    //                     "{}{}{}",
    //                     &current[..pos],
    //                     from,
    //                     &current[pos + to.len()..]
    //                 );
    //                 steps += 1;
    //                 break;  // Take first replacement we find
    //             }
    //         }
            
    //         // If no replacement was possible, we're stuck
    //         if previous == current {
    //             panic!("No valid reduction found!");
    //         }
    //     }
        
    //     steps
    // }

}

impl Day {
    fn find_predecessors(&self, molecule: &str) -> HashSet<String> {
        let mut results = HashSet::new();
        
        // For each (from, to_list) in our replacements
        for (from, to_list) in self.iter() {
            for to in to_list {
                // For each occurrence of 'to' in the molecule
                let mut start = 0;
                while let Some(pos) = molecule[start..].find(to) {
                    let actual_pos = start + pos;
                    
                    // Replace this occurrence with 'from'
                    let new_molecule = format!(
                        "{}{}{}",
                        &molecule[..actual_pos],
                        from,
                        &molecule[actual_pos + to.len()..]
                    );
                    results.insert(new_molecule);
                    
                    start = actual_pos + 1;
                }
            }
        }
        
        // dbg!(results)
        results
    }

    fn find_path_to_electron(&self) -> usize {
        let mut molecule = self.1.clone();
        let mut count = 0;
        
        while molecule != "e" {
            if let Some(next) = self.find_predecessors(&molecule)
                .into_iter()
                .min_by_key(|s| s.len()) {  // Prefer shorter molecules
                molecule = next;
                count += 1;
            } else {
                panic!("No valid reduction found!");
            }
        }
        
        count
    }
}

// impl Day {
//     fn count_steps(&self) -> usize {
//         let molecule = &self.1;
        
//         // Count specific patterns
//         let rn_count = molecule.matches("Rn").count();
//         let ar_count = molecule.matches("Ar").count();
//         let y_count = molecule.matches('Y').count();
        
//         // Count total atoms (uppercase followed by optional lowercase)
//         let atom_count = molecule.chars()
//             .enumerate()
//             .filter(|(i, c)| {
//                 c.is_uppercase() && (
//                     i + 1 >= molecule.len() ||
//                     !molecule.chars().nth(i + 1).unwrap_or(' ').is_lowercase()
//                 )
//             })
//             .count();
            
//         // The formula appears to be:
//         // atoms - rn - ar - 2*y - 1
//         atom_count - rn_count - ar_count - 2 * y_count - 1
//     }
// }

impl Day {
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
            
        println!("Molecule: {}", molecule);
        println!("Atoms: {}", atom_count);
        println!("Rn: {}", rn_count);
        println!("Ar: {}", ar_count);
        println!("Y: {}", y_count);
            
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
        // dbg!(&self);

        // let mut molecule = self.1.clone();
        // let mut steps = 0;

        // while molecule != "e" {
        //     for (from, to_list) in self.iter() {
        //         for to in to_list {
        //             if molecule.contains(to) {
        //                 molecule = molecule.replacen(to, from, 1);
        //                 steps += 1;
        //             }
        //         }
        //     }
        // }

        // Ok(steps)

        Ok(self.count_steps())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-19.txt"))?;
    // let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    // println!("Part 1: {}", part1);
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
