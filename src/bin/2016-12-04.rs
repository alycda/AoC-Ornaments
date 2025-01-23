//! # Day 4: Security Through Obscurity
//!

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, Solution};
use nom::{
    character::complete::{alpha1, char, digit1},
    multi::many1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

/// - ecrypted name
/// - room number
/// - checksum
#[derive(Debug, derive_more::Deref)]
struct Day(Vec<(usize, String, HashMap<char, usize>)>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(
            input
                .lines()
                .map(|line| Day::parse_line(line).unwrap().1)
                .collect(),
        ))
    }
}

impl Day {
    fn parse_line(input: &str) -> IResult<&str, (usize, String, HashMap<char, usize>)> {
        let (input, parts) = many1(terminated(alpha1, char('-')))(input)?;
        let map = Self::count_frequencies(&parts);
        let (_, (id, checksum)) = Self::parse_id_and_checksum(input)?;

        Ok((
            "",
            (id.parse::<usize>().expect("num"), checksum.to_string(), map),
        ))
    }

    fn parse_id_and_checksum(input: &str) -> IResult<&str, (&str, &str)> {
        tuple((digit1, delimited(char('['), alpha1, char(']'))))(input)
    }

    fn count_frequencies(items: &[&str]) -> HashMap<char, usize> {
        let mut freq_map: HashMap<char, usize> = HashMap::new();

        for &item in items {
            for c in item.chars() {
                *freq_map.entry(c).or_insert(0) += 1;
            }
        }

        freq_map
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // dbg!(&self);

        Ok(self
            .iter()
            .filter_map(|(id, checksum, map)| {
                // let a = map.iter().map(|(k, v)| k.to_string()).collect::<String>();
                // dbg!(map.iter().collect::<String>());

                // Convert to vec for sorting
                let mut freq_vec: Vec<_> = map.iter().collect();

                // Sort by value (count) descending, then by key for stable sorting
                freq_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

                let a = freq_vec
                    .iter()
                    .map(|(k, v)| k.to_string())
                    .collect::<String>();

                // if a == *checksum {
                if a.starts_with(checksum) {
                    return Some(id);
                }

                None
            })
            .sum())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-04.txt"))?;
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
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", 123)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]", 987)]
    #[case("not-a-real-room-404[oarel]", 404)]
    #[case("totally-real-room-200[decoy]", 0)]
    fn test_cases(#[case] input: &str, #[case] expected: usize) {
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
    }

    //     #[test]
    //     fn part_1() {
    //         let input = "aaaaa-bbb-z-y-x-123[abxyz]
    // a-b-c-d-e-f-g-h-987[abcde]
    // not-a-real-room-404[oarel]
    // totally-real-room-200[decoy]";

    //         let mut day = Day::from_str(input).unwrap();
    //         assert_eq!(day.solve(Part::One).unwrap(), "1514");
    //     }
}
