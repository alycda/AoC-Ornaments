//! Day 12: JSAbacusFramework.io

use std::{marker::PhantomData, str::FromStr};

use aoc_ornaments::{Part, Solution};
use nom::{bytes::complete::take_till, character::complete::{char, digit1}, combinator::{map, opt, recognize}, sequence::pair, IResult};
use serde_json::Value;

#[derive(Debug)]
struct Text;

/// intentionally mispelling to avoid ANY conflicts with serde_json
#[derive(Debug)]
struct Jasn;

#[derive(Debug)]
struct Day<J>(Vec<i64>, PhantomData<J>);

impl<J> std::ops::Deref for Day<J> {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day<Text> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut numbers = Vec::new();
        let mut input = input;

        while !input.is_empty() {
            match Day::<Text>::find_digit(input) {
                Ok((remainder, _)) => {
                    match Day::<Text>::parse_number(remainder) {
                        Ok((remainder, number)) => {
                            numbers.push(number);
                            input = remainder;
                        },
                        _ => break
                    }
                },
                _ => todo!()
            }
        }

        Ok(Self(numbers, PhantomData))
    }
}

impl FromStr for Day<Jasn> {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let json: Value = serde_json::from_str(input)
            .map_err(|e| miette::miette!("Failed to parse JSON: {}", e))?;
        
        // Part 1: Just collect all numbers
        let numbers = Day::<Jasn>::ignore_red(&json);

        Ok(Self(numbers, PhantomData))
    }
}

impl<J> Day<J> {
    fn find_digit(input: &str) -> IResult<&str, &str> {
        // take_till takes a predicate function that returns true when we should stop
        take_till(|c: char| c.is_ascii_digit() || c == '-')(input)
    }

    fn parse_number(input: &str) -> IResult<&str, i64> {
        map(
            recognize(
                pair(
                    opt(char('-')),
                    digit1,
                )
            ),
            |num_str: &str| num_str.parse().unwrap()
        )(input)
    }

    fn ignore_red(value: &Value) -> Vec<i64> {
        let mut numbers = Vec::new();
        match value {
            Value::Number(n) => {
                if let Some(n) = n.as_i64() {
                    numbers.push(n);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    numbers.extend(Self::ignore_red(v));
                }
            }
            Value::Object(obj) => {
                // Check if any value is the string "red"
                if obj.values().any(|v| v == "red") {
                    return Vec::new(); // Ignore this object and all children
                }
                for v in obj.values() {
                    numbers.extend(Self::ignore_red(v));
                }
            }
            _ => {}
        }
        numbers      
    }
}

impl<J> Solution for Day<J> where Day<J>: FromStr {
    type Output = i64;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.iter().sum())
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        Ok(self.iter().sum())
    }
}

fn main() -> miette::Result<()> {
    let input = include_str!("../inputs/2015-12-12.txt");
    let mut day_part1 = Day::<Text>::from_str(input)?;
    let mut day_part2 = Day::<Jasn>::from_str(input)?;
    let part1 = day_part1.solve(Part::One)?;
    let part2 = day_part2.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: i64) {
        let mut day = Day::<Text>::from_str(input).unwrap();
        let output = day.solve(Part::One).unwrap();

        assert_eq!(output, expected.to_string());
    }

    #[rstest]
    #[case("[1,2,3]", 6)]
    #[case(r#"[1,{"c":"red","b":2},3]"#, 4)]
    #[case(r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0)]
    #[case(r#"[1,"red",5]"#, 6)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: i64) {
        let mut day = Day::<Jasn>::from_str(input).unwrap();
        let output = day.solve(Part::Two).unwrap();

        assert_eq!(output, expected.to_string());
    }
}