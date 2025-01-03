//! Day 11: Corporate Policy

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
    fn increment_password(password: &str) -> String {
        let mut chars: Vec<char> = password.chars().collect();
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

        chars.iter().collect()
    }

    fn next_password(current: &str) -> String {
        let mut password = String::from(current);
        
        loop {
            password = Self::increment_password(&password);
            if Self::is_valid_password(&password) {
                break;
            }
        }
        
        password
    }

    /// Rule 1: Must have an increasing straight of 3 letters
    fn three_consecutive(input: &[u8]) -> bool {
        input.windows(3).any(|window| {
            window[0] + 1 == window[1] && window[1] + 1 == window[2]
        })
    }

    /// Rule 2: Must not contain i, o, or l
    fn has_forbidden_characters(input: &str) -> bool {
        input.chars().any(|c| "iol".contains(c))
    }

    /// Rule 3: Must have at least two different pairs
    fn has_two_pairs(chars: Vec<char>) -> bool {
        let mut pairs = Vec::new();
        let mut i = 0;
        while i < chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                pairs.push(chars[i]);
                i += 2;
            } else {
                i += 1;
            }
        }

        pairs.len() >= 2 && pairs[0] != pairs[1]
    }

    fn is_valid_password(password: &str) -> bool {    
        Self::three_consecutive(password.as_bytes()) 
            && !Self::has_forbidden_characters(password) 
            && Self::has_two_pairs(password.chars().collect())
    }

}

impl Solution for Day {
    type Output = String;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(Day::next_password(&self.iter().collect::<String>()))
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let part1 = self.part1()?;
        Ok(Day::next_password(&part1))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-11.txt"))?;
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
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    fn test_cases_invalid_part1(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(Day::is_valid_password(input), expected);
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