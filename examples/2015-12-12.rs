//! Day 12: JSAbacusFramework.io

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
// use miette::Error;
use nom::{bytes::complete::take_till, character::complete::{char, digit1}, combinator::{map, opt, recognize}, sequence::pair, IResult};

#[derive(Debug)]
struct Day(Vec<i64>);

impl std::ops::Deref for Day {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut numbers = Vec::new();
        let mut input = input;

        while !input.is_empty() {
            match Day::find_digit(input) {
                Ok((remainder, _)) => {
                    match Day::parse_number(remainder) {
                        Ok((remainder, number)) => {
                            numbers.push(number);
                            input = remainder;
                        },
                        _ => break
                        // Err(x) => todo!("x: {x:?}")
                        // _ => todo!()
                    }
                    // let (remainder, number) = Day::parse_number(remainder).unwrap();
                    // numbers.push(number);
                    // input = remainder;
                },
                _ => todo!()
            }
        }


        // let a = take_until::<_, _, nom::error::Error<_>>("0123456789")(input)
        //     .map_err(|e| miette::miette!("Failed to parse input: {}", e))
        //     .map(|(_, output)| output);

        // // let something = take_until::<&str, &str, Error>("0123456789")(input)
        // //     .unwrap();
        //     // .map_err(|e| miette::miette!("Failed to parse input: {}", e));
        //     // .map(|(_, output)| output);

        // dbg!(a);

        // dbg!(Day::nom_parser().unwrap());

        // dbg!(Day::parse_number(input));

        // let (a, _) = Day::better_approach(input).unwrap();

        // // dbg!(Day::better_approach(input).unwrap());

        // // dbg!(a);

        // let (b, c) = Day::parse_number(a).unwrap();

        // dbg!(b, c);

        // todo!()

        Ok(Self(numbers))
    }
}

impl Day {
    // fn nom_parser() -> IResult<&'static str, &'static str> {
    //     let (input, _) = take_until::<_, _, nom::error::Error<_>>("0123456789")("abc123").unwrap();
    //     let (input, output) = digit1::<_, nom::error::Error<_>>(input).unwrap();

    //     Ok((input, output))
    // }

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
}

impl Solution for Day {
    type Output = i64;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        // dbg!(self);

        // // let mut sum = Vec::new();

        // // Ok(sum.len())

        // todo!()

        Ok(self.iter().sum())
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-12.txt"))?;
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
    #[case("[1,2,3]", 6)]
    #[case(r#"{"a":2,"b":4}"#, 6)]
    #[case("[[[3]]]", 3)]
    #[case(r#"{"a":{"b":4},"c":-1}"#, 3)]
    #[case(r#"{"a":[-1,1]}"#, 0)]
    #[case("[]", 0)]
    #[case("{}", 0)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: i64) {
        let mut day = Day::from_str(input).unwrap();
        let output = day.solve(Part::One).unwrap();

        assert_eq!(output, expected.to_string());
    }


}