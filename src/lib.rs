pub trait Solution: TryFrom<&'static str> {
    /// Ensures the output can be converted to a string
    type Output: std::fmt::Display + Default;  

    /// Required for AoC
    fn part1(&mut self) -> SolutionResult<<Self as Solution>::Output> {
        todo!()
    }

    /// Required for AoC
    fn part2(&mut self) -> SolutionResult<<Self as Solution>::Output> {
        todo!()
    }

    /// Optional, for everybody.codes or bonus AoC
    fn part3(&mut self) -> SolutionResult<<Self as Solution>::Output> {
        Ok(<Self as Solution>::Output::default())
    }

    fn solve(&mut self, which: Part) -> SolutionResult<String> {
        Ok(match which {
            Part::One => self.part1()?.to_string(),
            Part::Two => self.part2()?.to_string(),
            Part::Three => self.part3()?.to_string(),
        })
    }
}

/// Puzzle parts
#[derive(Debug, Clone, Copy)]
pub enum Part {
    /// AoC
    One,
    /// AoC
    Two,
    /// for everybody.codes
    Three,
}

#[cfg(feature = "miette")]
pub type SolutionResult<T> = miette::Result<T>;
#[cfg(not(feature = "miette"))]
pub type SolutionResult<T> = Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    use super::*;

    struct Day;

    impl Solution for Day {
        type Output = String;

        fn part1(&mut self) -> miette::Result<Self::Output> {
            Ok("Hello, Rudolph!".into())
        }

        fn part2(&mut self) -> miette::Result<Self::Output> {
            Ok("Hello, Santa!".into())
        }
    }

    impl TryFrom<&str> for Day {
        type Error = miette::Error;

        fn try_from(_input: &str) -> miette::Result<Self> {
            Ok(Self)
        }
    }

    #[test]
    fn test_part_1() -> miette::Result<()> {
        let mut day = Day::try_from("")?;

        let solution = day.solve(Part::One)?;
        assert_eq!("Hello, Rudolph!".to_string(), solution);
        Ok(())
    }

    #[test]
    fn test_part_2() -> miette::Result<()> {
        let mut day: Day = "".try_into()?;

        let solution = day.solve(Part::Two)?;
        assert_eq!("Hello, Santa!".to_string(), solution);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_part_3() {
        let mut day = Day;

        let solution = day.solve(Part::Three).unwrap();
        assert_eq!("Hello, Dasher!".to_string(), solution);
    }
}
