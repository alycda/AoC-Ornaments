//! Day 1: Not Quite Lisp

use aoc_ornaments::{Part, Solution};

struct Day(i32);

impl std::ops::Deref for Day {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solution for Day {
    type Output = i32;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        Ok(**self)
    }
}

impl TryFrom<&str> for Day {
    type Error = miette::Error;

    fn try_from(input: &str) -> miette::Result<Self> {
        let parsed = input.chars().map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            }
        }).sum();

        Ok(Self(parsed))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::try_from(include_str!("./inputs/2015-12-01.txt"))?;
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
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_cases(#[case] input: &str, #[case] expected: i32) {
        let mut day = Day::try_from(input).unwrap();
        assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
    }
}
