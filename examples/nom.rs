//! This example demonstrates how to use the `nom` parser combinators library to parse input data.

use std::str::FromStr;

// use aoc_ornaments::Solution;

struct Day(&'static str);

// impl Solution for Day {
//     type Output = String;

//     fn part1(&mut self) -> miette::Result<Self::Output> {
//         Ok("Hello, Rudolph!".into())
//     }

//     fn part2(&mut self) -> miette::Result<Self::Output> {
//         Ok("Hello, Santa!".into())
//     }
// }

impl Day {
    fn nom_parser(input: &'static str) -> nom::IResult<&'static str, &'static str> {
        Ok((input, ""))
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        // We leak the input to get a 'static str - this IS a bad practice! shown only for the purpose of this example (to avoid lifetimes & new allocation)
        let static_input = Box::leak(input.to_string().into_boxed_str());

        // otherwise you will get: error[E0521]: borrowed data escapes outside of associated function
        let (_, data) = Self::nom_parser(static_input)
            .map_err(|e| miette::miette!("Failed to parse input: {}", e))?;

        Ok(Self(data))
    }
}

fn main() -> miette::Result<()>{
    Day::from_str("")?;
    
    Ok(())
}