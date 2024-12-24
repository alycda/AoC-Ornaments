//! Example of how to implement different parsers for each part of the solution.

use std::{marker::PhantomData, str::FromStr};

use aoc_ornaments::{Part, Solution};

struct Part1;
struct Part2;
// we can have as many parts as we like, but usually only have two(2) (AoC vs everybody.codes)
// struct Part3;

/// In this heavily contrived example, I'm using a tuple struct ONLY to hold the type for the compiler.
/// In the real world, this will likely be a normal struct so that it can properly hold the data you parsed.
struct Day<P>(PhantomData<P>);

impl<P> Solution for Day<P> 
where
    Day<P>: FromStr<Err = miette::Error>,
{
    type Output = usize;
}

// blanket implementation will cause conflicts for the compiler, instead you must add the where clause above
// impl<P> FromStr for Day<P> {
//     type Err = miette::Error;

//     fn from_str(_input: &str) -> miette::Result<Self> {
//         Ok(Self(PhantomData))
//     }
// } 

impl FromStr for Day<Part1> {
    type Err = miette::Error;

    fn from_str(_input: &str) -> miette::Result<Self> {
        Ok(Self(PhantomData))
    }
}

impl FromStr for Day<Part2> {
    type Err = miette::Error;

    fn from_str(_input: &str) -> miette::Result<Self> {
        Ok(Self(PhantomData))
    }
}


fn main() -> miette::Result<()> {
    let input = "aoc";

    let part1 = Day::<Part1>::from_str(input)?.solve(Part::One)?;
    let part2 = Day::<Part2>::from_str(input)?.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}