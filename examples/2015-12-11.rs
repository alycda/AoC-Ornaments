//! Day 11: Corporate Policy

use core::str;
use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

struct Day(Vec<char>);

impl std::ops::Deref for Day {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::MietteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

impl Day {
    fn three_consecutive(input: &Vec<char>) -> bool {
        let mut peekable = input.iter().peekable();
        let mut count = 0;

        while let Some(c) = peekable.next() {
            if let Some(next) = peekable.peek() {
                if *c as u8 + 1 == **next as u8 {
                    count += 1;
                } else {
                    count = 0;
                }
            }
        }

        count > 2
    }

    fn has_forbidden_characters(input: &str) -> bool {
        input.contains('i') || input.contains('o') || input.contains('l')
    }

    fn two_pairs(input: &Vec<char>) -> bool {
        let mut pairs = 0;
        // let mut current_char = input[0];
        let mut peekable = input.iter().peekable();

        while let Some(c) = peekable.next() {
            if let Some(next) = peekable.peek() {
                if &c == next {
                    pairs += 1;
                }
            }
        }


        pairs > 1
    }

    fn next_pw(chars: &mut Vec<char>) {
        let mut i = chars.len() - 1;
        loop {
            if chars[i] == 'z' {
                chars[i] = 'a';
                if i == 0 {
                    break;
                }
                i -= 1;
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                break;
            }
        }
    }
}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let mut password = self.clone();
        let str_password = password.iter().collect::<String>();

        while !Day::three_consecutive(&password) || Day::has_forbidden_characters(&str_password) || !Day::two_pairs(&password) {
            Day::next_pw(&mut password);
        }

        Ok(password.iter().collect())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-11.txt"))?;
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
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    fn test_cases_is_valid_part1(#[case] input: &str, #[case] expected: bool) {
        assert!(Day::has_forbidden_characters(input) || !Day::three_consecutive(&input.chars().collect()) || !Day::two_pairs(&input.chars().collect()) == expected);
    }

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_cases_next_pw_part1(#[case] input: &str, #[case] expected: &str) {
        let mut day = Day::from_str(input).unwrap();
        let part1 = day.solve(Part::One).unwrap();

        assert_eq!(part1, expected);
    }
}