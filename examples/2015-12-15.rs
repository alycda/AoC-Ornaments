//! Day 15: Science for Hungry People

use std::{collections::HashMap, ops::Add, str::FromStr};

use aoc_ornaments::{Part, Solution};
use itertools::Itertools;
use nom::{bytes::complete::{tag, take_until}, character::complete::{i32, space0, space1}, multi::separated_list1, sequence::{delimited, preceded, terminated, tuple}, IResult};

type Ingredients = HashMap<String, Properties>;

#[derive(Debug, Clone, Copy)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Properties {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, props) = Day::parse_properties(input).map_err(|e| miette::miette!("Failed to parse properties: {}", e))?;

        Ok(props)
    }
}

impl From<(i32, i32, i32, i32, i32)> for Properties {
    fn from((capacity, durability, flavor, texture, calories): (i32, i32, i32, i32, i32)) -> Self {
        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

impl From<(&str, &str, &str, &str, &str)> for Properties {
    fn from((capacity, durability, flavor, texture, calories): (&str, &str, &str, &str, &str)) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }
}

impl From<[&str; 5]> for Properties {
    fn from([capacity, durability, flavor, texture, calories]: [&str; 5]) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }
}

impl Add for Properties {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories
        }
    }
}

impl Properties {
    fn new(capacity: &str, durability: &str, flavor: &str, texture: &str, calories: &str) -> Self {
        Self {
            capacity: capacity.parse().expect("a number"),
            durability: durability.parse().expect("a number"),
            flavor: flavor.parse().expect("a number"),
            texture: texture.parse().expect("a number"),
            calories: calories.parse().expect("a number"),
        }
    }

    fn teaspoon(&self, count: usize) -> Self {
        // self.capacity * self.durability * self.flavor * self.texture

        Self {
            capacity: self.capacity * count as i32,
            durability: self.durability * count as i32,
            flavor: self.flavor * count as i32,
            texture: self.texture * count as i32,
            calories: self.calories * count as i32,
        }
    }

    fn score(&self) -> usize {
        let capacity = self.capacity.max(0) as usize;
        let durability = self.durability.max(0) as usize;
        let flavor = self.flavor.max(0) as usize;
        let texture = self.texture.max(0) as usize;

        capacity * durability * flavor * texture
    }
}

impl From<Vec<(&str, i32)>> for Properties {
    fn from(props: Vec<(&str, i32)>) -> Self {
        // We can use try_into to convert the Vec to an array
        let arr: [(&str, i32); 5] = props.try_into()
            .expect("Expected exactly 5 properties");
            
        // Now we need to arrange the values in the correct order
        // Since the property names could come in any order, we should match on them
        let mut capacity = 0;
        let mut durability = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calories = 0;

        for (name, value) in arr {
            match name {
                "capacity" => capacity = value,
                "durability" => durability = value,
                "flavor" => flavor = value,
                "texture" => texture = value,
                "calories" => calories = value,
                _ => panic!("Unknown property: {}", name),
            }
        }

        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[derive(Debug)]
struct Day(Ingredients);

impl std::ops::Deref for Day {
    type Target = Ingredients;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.lines().fold(Ingredients::new(), |mut map, line| {
            let (_, (name, props)) = Self::parse_ingredient(line).expect("valid input");

            map.insert(name, props);

            map
        })))
    }
}

impl Day {
    fn parse_ingredient(input: &str) -> IResult<&str, (String, Properties)> {
        let (list, name) = terminated(
            // This will capture everything until the colon
            take_until(":"),
            // This consumes the colon and any following whitespace
            tuple((tag(":"), space0))
        )(input)?;

        // dbg!(name, list);

        let (remainder, props) = Self::parse_properties(list)?;
        // dbg!(&remainder);

        Ok((remainder, (name.to_string(), props)))
    }

    fn parse_properties(input: &str) -> IResult<&str, Properties> {
        let (remainder, props) = separated_list1(
            delimited(space0, tag(","), space0),  // This handles " , " as separator
            Self::parse_property
        )(input)?;

        Ok((remainder, Properties::from(props)))
    }

    // Parse a single property like "capacity 2"
    fn parse_property(input: &str) -> IResult<&str, (&str, i32)> {
        tuple((
            // Property name
            take_until(" "),
            // Space followed by number
            preceded(space1, i32)
        ))(input)
    }

    fn score_recipe(&self, amounts: &[(String, usize)]) -> usize {
        // Sum up the properties for each ingredient according to its amount
        let total_properties = amounts.iter().fold(
            Properties {
                capacity: 0,
                durability: 0,
                flavor: 0,
                texture: 0,
                calories: 0,
            },
            |acc, (name, amount)| {
                acc + self.0[name].teaspoon(*amount)
            }
        );
        
        total_properties.score()
    }

    fn calories(&self, amounts: &[(String, usize)]) -> i32 {
        amounts.iter().fold(0, |acc, (name, amount)| {
            acc + self.0[name].calories * *amount as i32
        })
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let ingredients: Vec<String> = self.0.keys().cloned().collect();
        let n = ingredients.len();
        
        // Generate splits points for 100 teaspoons among n ingredients
        (0..100 + n - 1).combinations(n - 1)
            .map(|splits| {
                // Convert split points to amounts
                let mut amounts = Vec::with_capacity(n);
                let mut prev = -1;
                
                for (idx, &split) in splits.iter().enumerate() {
                    amounts.push((
                        ingredients[idx].clone(),
                        (split as i32 - prev - 1) as usize
                    ));
                    prev = split as i32;
                }
                
                // Handle last ingredient
                amounts.push((
                    ingredients[n-1].clone(),
                    (100 + n as i32 - 1 - prev - 1) as usize
                ));
                
                self.score_recipe(&amounts)
            })
            .max()
            .ok_or_else(|| miette::miette!("No valid combinations found"))
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        let ingredients: Vec<String> = self.0.keys().cloned().collect();
        let n = ingredients.len();
        
        // Generate splits points for 100 teaspoons among n ingredients
        (0..100 + n - 1).combinations(n - 1)
            .map(|splits| {
                // Convert split points to amounts
                let mut amounts = Vec::with_capacity(n);
                let mut prev = -1;
                
                for (idx, &split) in splits.iter().enumerate() {
                    amounts.push((
                        ingredients[idx].clone(),
                        (split as i32 - prev - 1) as usize
                    ));
                    prev = split as i32;
                }
                
                // Handle last ingredient
                amounts.push((
                    ingredients[n-1].clone(),
                    (100 + n as i32 - 1 - prev - 1) as usize
                ));

                if self.calories(&amounts) != 500 {
                    return 0;
                }
                
                self.score_recipe(&amounts)
            })
            .max()
            .ok_or_else(|| miette::miette!("No valid combinations found"))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-15.txt"))?;
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
    #[case((44, 56), (68, 80, 152, 76))]
    fn test_cases_part1(#[case] tsp: (usize, usize), #[case] expected: (usize, usize, usize, usize)) {
        let butterscotch = Properties::from_str("capacity -1, durability -2, flavor 6, texture 3, calories 8").expect("invalid input");
        let cinnamon = Properties::from_str("capacity 2, durability 3, flavor -2, texture -1, calories 3").expect("invalid input");

        let ingredients = dbg!(butterscotch.teaspoon(tsp.0) + cinnamon.teaspoon(tsp.1));

        assert_eq!(ingredients.score(), expected.0 * expected.1 * expected.2 * expected.3);
    }

    #[test]
    fn test_part1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "62842880");
    }

    #[rstest]
    #[case((40, 60), (500, 57600000))]
    fn test_cases_part2(#[case] tsp: (usize, usize), #[case] expected: (i32, usize)) {
        let butterscotch = Properties::from_str("capacity -1, durability -2, flavor 6, texture 3, calories 8").expect("invalid input");
        let cinnamon = Properties::from_str("capacity 2, durability 3, flavor -2, texture -1, calories 3").expect("invalid input");

        let ingredients = dbg!(butterscotch.teaspoon(tsp.0) + cinnamon.teaspoon(tsp.1));

        assert_eq!(ingredients.calories, expected.0);
        assert_eq!(ingredients.score(), expected.1);
    }
}