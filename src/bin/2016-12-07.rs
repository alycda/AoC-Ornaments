//! # Day 7: Internet Protocol Version 7
//!

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, multi::many1,
    sequence::delimited, IResult,
};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<Vec<String>>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Day(input
            .lines()
            .map(|line| {
                let (_, sequences) = Self::parse_line(line).unwrap();

                sequences.iter().map(|x| String::from(*x)).collect()
            })
            .collect::<Vec<_>>()))
    }
}

impl Day {
    /// sequences, hypernet sequences
    fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
        many1(alt((delimited(tag("["), alpha1, tag("]")), alpha1)))(input)
    }

    fn supports_tls(input: &str) -> bool {
        input
            .chars()
            .collect::<Vec<_>>()
            .windows(4)
            .any(|x| x[0] == x[3] && x[0] != x[1] && x[1] == x[2])
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self
            .iter()
            .filter(|ipv7| {
                let (left, right): (Vec<_>, Vec<_>) =
                    ipv7.iter().enumerate().partition(|(idx, _)| idx % 2 == 0);

                let sequence: Vec<_> = left.into_iter().map(|(_, s)| s).collect();
                let hypernet: Vec<_> = right.into_iter().map(|(_, s)| s).collect();

                let any_result = sequence.iter().any(|s| Day::supports_tls(s));
                let all_result = hypernet.iter().all(|s| !Day::supports_tls(s));

                any_result && all_result
            })
            .count())
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2016-12-07.txt"))?;
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
    #[case("abba[mnop]qrst", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn test_cases(#[case] input: &str, #[case] expected: bool) {
        let mut day = Day::from_str(input).unwrap();

        assert_eq!(day.solve(Part::One).unwrap() == "1", expected);
    }
}
