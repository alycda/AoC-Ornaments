//! Example of how to implement different parsers for each part of the solution.

use std::marker::PhantomData;

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
    Day<P>: TryFrom<&'static str, Error = miette::Error>,
{
    type Output = usize;
}

// blanket implementation will cause conflicts for the compiler, instead you must add the where clause above
// impl<P> TryFrom<&str> for Day<P> {
//     type Error = miette::Error;

//     fn try_from(_input: &str) -> miette::Result<Self> {
//         Ok(Self(PhantomData))
//     }
// } 

impl TryFrom<&str> for Day<Part1> {
    type Error = miette::Error;

    fn try_from(_input: &str) -> miette::Result<Self> {
        Ok(Self(PhantomData))
    }
}

impl TryFrom<&str> for Day<Part2> {
    type Error = miette::Error;

    fn try_from(_input: &str) -> miette::Result<Self> {
        Ok(Self(PhantomData))
    }
}


fn main() -> miette::Result<()> {
    let input = "aoc";

    let part1 = Day::<Part1>::try_from(input)?.solve(Part::One)?;
    let part2 = Day::<Part2>::try_from(input)?.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}