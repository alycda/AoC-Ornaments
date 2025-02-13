//! Day 5: Doesn't He Have Intern-Elves For This?

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};

#[derive(Debug, derive_more::Deref)]
struct Day(Vec<String>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().map(str::to_string).collect()))
    }
}

impl Day {
    /// Check if a string is nice.
    /// 
    /// - it contains at least three vowels (aeiou)
    /// - a double letter (like xx)
    /// - does not contain the strings ab, cd, pq, or xy.
    /// 
    fn is_nice(line: &str) -> bool {
        !Self::has_forbidden_pair(line) && Self::has_double_letter(line) && (Self::count_vowels(line) >= 3)
    }

    /// Check if a string is nice using the new rules.
    /// 
    /// - it contains a pair of any two letters that appears at least twice in the string without overlapping
    /// - it contains at least one letter which repeats with exactly one letter between them
    /// 
    fn is_nice_v2(line: &str) -> bool {
        Self::has_non_overlapping_pair(line) && Self::has_sandwich_letter(line)
    }

    fn compute(&self, f: fn(&str) -> bool) -> usize {
        self.iter().filter(|line| f(line)).count()
    }

    fn has_non_overlapping_pair(s: &str) -> bool {
        let chars: Vec<_> = s.chars().collect();
        chars.windows(2).enumerate().any(|(i, w1)| {
            chars[i+2..].windows(2).any(|w2| w1[0] == w2[0] && w1[1] == w2[1])
        })
    }
    
    fn has_sandwich_letter(s: &str) -> bool {
        s.as_bytes().windows(3).any(|w| w[0] == w[2])
    }
    
    fn has_forbidden_pair(s: &str) -> bool {
        ["ab", "cd", "pq", "xy"].iter().any(|&pair| s.contains(pair))
    }
    
    fn has_double_letter(s: &str) -> bool {
        s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
    }
    
    fn count_vowels(s: &str) -> usize {
        s.chars().filter(|&c| Self::is_vowel(c)).count()
    }
    
    fn is_vowel(c: char) -> bool {
        matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
    }
    
}

impl Solution for Day {
    type Output = usize;

    /// Count the number of nice strings in the input.
    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Day::is_nice))
    }

    /// Count the number of nice strings in the input using the new rules.
    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        Ok(self.compute(Day::is_nice_v2))
    }
}

/// Run Part 1 and Part 2.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-05.txt"))?;
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
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_cases_part1(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(Day::is_nice(input), expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_cases_part2(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(Day::is_nice_v2(input), expected);
    }
}