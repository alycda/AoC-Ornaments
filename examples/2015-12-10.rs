//! Day 10: Elves Look, Elves Say

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;

/// char, count
#[derive(Debug)]
struct Day(Vec<(u64, char)>);

impl std::ops::Deref for Day {
    type Target = Vec<(u64, char)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut peekable = input.chars().peekable();
        let mut current_count = 1;
        let mut result = Vec::new();

        while let Some(c) = peekable.next() {
            // dbg!(current_count, c, peekable.peek());

            if let Some(next) = peekable.peek() {
                match(c, next) {
                    (c, next) if c == *next => {
                        current_count += 1;
                    }
                    (c, next) => {
                        result.push((current_count, c));
            //             dbg!(c, next);
                        current_count = 1
                    }
                }
            } else {
                // dbg!(current_count, c);
                result.push((current_count, c));
                // panic!("now what?");
            }

        }

        Ok(Self(result))

        // Ok(Self(peekable.fold(Vec::new(), |mut acc, c| {
        //     let count = acc.iter().rev().take_while(|(c2, _)| *c2 == c as u32).count();
        //     acc.push((c as u32, count));
        //     acc
        // })))
    }
}

impl Day {
    // fn next_sequence(&mut self) -> Self {
    //     let mut result = Vec::new();
    //     for &(count, digit) in self.0.iter() {
    //         // Push the count as a separate number, then the digit
    //         result.push((1, char::from_digit(count as u32, 10).unwrap()));
    //         result.push((1, digit));
    //     }
    //     Self(result)
    // }

    // fn next_sequence(&mut self) -> Self {
    //     let mut result = Vec::with_capacity(self.0.len() * 2); // Preallocate as it tends to grow
        
    //     for &(count, digit) in self.0.iter() {
    //         // Convert the count to a character (will be '1' to '9')
    //         let count_char = char::from_digit(count as u32, 10).unwrap();
    //         result.push((1, count_char));
    //         result.push((1, digit));
    //     }
        
    //     Self(result)
    // }

    fn next_sequence(&self) -> Self {
        // Convert our current state to a string to use group_by
        let string_rep = self.to_string();
        
        // Use group_by to collect runs of same digit
        let result = string_rep.chars()
            .group_by(|&x| x) // Group consecutive same chars
            .into_iter()
            .map(|(c, group)| {
                let count = group.count() as u64;
                (count, c)
            })
            .collect();

        Self(result)
    }

    // fn next_sequence_cached(&self) -> Self {
    //     let mut result_map: HashMap<(u64, char), u64> = HashMap::new();
        
    //     for &(count, digit) in self.0.iter() {
    //         // Instead of creating new vec entries, we'll increment counts in the map
    //         *result_map.entry((1, char::from_digit(count as u32, 10).unwrap())).or_default() += 1;
    //         *result_map.entry((1, digit)).or_default() += 1;
    //     }
        
    //     // Convert back to vec only once at the end
    //     let result = result_map
    //         .into_iter()
    //         .map(|((count, digit), occurrences)| (count * occurrences, digit))
    //         .collect();
            
    //     Self(result)
    // }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // dbg!(&self);

        let mut current = Day(self.clone());
        for _ in 0..39 {
            current = current.next_sequence();
        }
        Ok(current.0.len() * 2)

        // Ok(self.len() * 2)
    }
}

impl ToString for Day {
    fn to_string(&self) -> String {
        self.iter().map(|(count, digit)| format!("{}{}", count, digit)).collect()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-10.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); // > 45370, < 643280
    // println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", 11)]
    #[case("11", 21)]
    #[case("21", 1211)]
    #[case("1211", 111221)]
    #[case("111221", 312211)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: i32) {
        let mut day = Day::from_str(input).unwrap();

        assert_eq!(day.to_string(), expected.to_string());
    }
}