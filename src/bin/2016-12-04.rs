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
struct Day(Vec<(usize, String, String)>);

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
    fn parse_line(input: &str) -> IResult<&str, (usize, String, String)> {
        let (input, parts) = many1(terminated(alpha1, char('-')))(input)?;
        let cipher = parts.join("-");
        let (_, (id, checksum)) = Self::parse_id_and_checksum(input)?;

        Ok((
            "",
            (
                id.parse::<usize>().expect("num"),
                checksum.to_string(),
                cipher,
            ),
        ))
    }

    fn parse_id_and_checksum(input: &str) -> IResult<&str, (&str, &str)> {
        tuple((digit1, delimited(char('['), alpha1, char(']'))))(input)
    }

    fn count_frequencies(item: &str) -> HashMap<char, usize> {
        let mut freq_map: HashMap<char, usize> = HashMap::new();

        // for &item in items {
        for c in item.chars() {
            if c != '-' {
                *freq_map.entry(c).or_insert(0) += 1;
            }
        }
        // }

        freq_map
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        Ok(self
            .iter()
            .filter_map(|(id, checksum, cipher)| {
                let map = Self::count_frequencies(cipher);

                // Convert to vec for sorting
                let mut freq_vec: Vec<_> = map.iter().collect();

                // Sort by value (count) descending, then by key for stable sorting
                freq_vec.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

                let a = freq_vec
                    .iter()
                    .map(|(k, v)| k.to_string())
                    .collect::<String>();

                if a.starts_with(checksum) {
                    return Some(id);
                }

                None
            })
            .sum())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self
            .iter()
            .find(|(id, _, cipher)| {
                let name = cipher
                    .bytes()
                    .map(|b| {
                        if b.is_ascii_lowercase() {
                            // Convert to 0-25 range
                            let base = b - b'a';
                            let shift = id % 26;

                            let shifted = (base as i16 + shift as i16).rem_euclid(26) as u8;
                            // Convert back to ascii range
                            shifted + b'a'
                        } else if b == b'-' {
                            b' '
                        } else {
                            b
                        }
                    })
                    .map(|b| b as char)
                    .collect::<String>();

                // dbg!(&name) == "northpole object storage"
                &name == "northpole object storage"
            })
            .unwrap()
            .0)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-04.txt"))?;
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
    #[case("aaaaa-bbb-z-y-x-123[abxyz]", 123)]
    #[case("a-b-c-d-e-f-g-h-987[abcde]", 987)]
    #[case("not-a-real-room-404[oarel]", 404)]
    #[case("totally-real-room-200[decoy]", 0)]
    fn test_cases(#[case] input: &str, #[case] expected: usize) {
        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
    }

    #[test]
    fn part_1() {
        let input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";

        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), "1514");
    }

    // #[rstest]
    // #[case("qzmt-zixmtkozy-ivhz-343[zzzab]", "very encrypted name")]
    // fn part_2(#[case] input: &str, #[case] expected: &str) {
    //     let mut day = Day::from_str(input).unwrap();
    //     assert_eq!(day.solve(Part::Two).unwrap(), expected.to_string());
    // }
}
