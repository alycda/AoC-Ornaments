//! Day 8: Matchsticks

use std::fs;
use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug)]
struct Day(Vec<String>);

impl std::ops::Deref for Day {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().map(|line| line.to_string()).collect()))
    }
}

impl Day {
    fn process_string(input: &str) -> StringMetrics {
        let code_len = input.len();
        let mut mem_len = 0;
        let mut chars = input[1..input.len()-1].chars(); // Skip outer quotes
        
        while let Some(c) = chars.next() {
            match c {
                '\\' => {
                    match chars.next() {
                        Some('\\') | Some('"') => mem_len += 1,
                        Some('x') => {
                            // Skip two hex chars
                            chars.next();
                            chars.next();
                            mem_len += 1;
                        }
                        _ => mem_len += 1,
                    }
                }
                _ => mem_len += 1,
            }
        }
    
        StringMetrics { code_len, mem_len }
    }

    fn encode_string(input: &str) -> String {
        let mut result = String::with_capacity(input.len() + 4);
        result.push('"');
        
        for c in input.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                _ => result.push(c),
            }
        }
        
        result.push('"');
        result
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        let total = self.iter()
            .map(|line| {
                let metrics = Self::process_string(line);
                metrics.code_len - metrics.mem_len
            })
            .sum::<usize>();
        
        Ok(total)
    }

    fn part2(&mut self) -> miette::Result<Self::Output> {
        let total = self.iter()
            .map(|line| {
                let encoded = Self::encode_string(line);
                encoded.len() - line.len()
            })
            .sum::<usize>();
        
        Ok(total)
    }
}


#[derive(Debug)]
struct StringMetrics {
    code_len: usize,
    mem_len: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // include_str! will add even more escape sequences and break the code
    let input = fs::read_to_string("./examples/inputs/2015-12-08.txt")?;
    let mut day = Day::from_str(&input)?;
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
    #[case(r#""""#, (2, 0))]
    #[case(r#""abc""#, (5, 3))]
    #[case(r#""aaa\"aaa""#, (10, 7))]
    #[case(r#""\x27""#, (6, 1))]
    fn test_cases_part1(#[case] input: &str, #[case] expected: (usize, usize)) {
        let metrics = Day::process_string(input);
        assert_eq!(metrics.code_len, expected.0);
        assert_eq!(metrics.mem_len, expected.1);
    }

    fn test_part_1() {
        let input = r#"##
"abc"
"aaa\"aaa"
"\x27""#;

        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), "12");
    }

    #[rstest]
    #[case(r#""""#, 6)]
    #[case(r#""abc""#, 9)]
    #[case(r#""aaa\"aaa""#, 16)]
    #[case(r#""\x27""#, 11)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: usize) {
        let new_string = Day::encode_string(input);
        assert_eq!(new_string.len(), expected);
    }

    fn test_part_2() {
        let input = r#"##
"abc"
"aaa\"aaa"
"\x27""#;

        let mut day = Day::from_str(input).unwrap();
        assert_eq!(day.solve(Part::Two).unwrap(), "19");
    }
}