//! Day 15: Science for Hungry People

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{Part, Solution};
use nom::{bytes::complete::{tag, take_until, take_while}, character::complete::{i32, space0, space1}, multi::separated_list1, sequence::{delimited, preceded, terminated, tuple}, IResult};

type Ingredients = HashMap<String, Properties>;

#[derive(Debug)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
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

        dbg!(name, list);

        let (remainder, props) = Self::parse_properties(list)?;

        Ok((remainder, (name.to_string(), props)))
    }

    fn parse_properties(input: &str) -> IResult<&str, Properties> {
        // dbg!(input);

        // let (x, y) = terminated(take_until("\n"), take_while(separated_list1(tag(","), Self::parse_property)))(input);
        // let (x, y) = terminated(take_until("\n"), take_while(|| separated_list1(tag(","), Self::parse_property)))(input);

        // dbg!(x, y);

        // let (a, b) = separated_list1(tag(","), Self::parse_property)(input)?;
        let (_, props) = separated_list1(
            delimited(space0, tag(","), space0),  // This handles " , " as separator
            Self::parse_property
        )(input)?;

        dbg!(&props);

        // let (remainder, (
        //     (_, capacity), _, // capacity and comma
        //     (_, durability), _,
        //     (_, flavor), _,
        //     (_, texture), _,
        //     (_, calories)
        // )) = tuple((
        //     tuple((Self::parse_property, tag(", "))),
        //     tuple((Self::parse_property, tag(", "))),
        //     tuple((Self::parse_property, tag(", "))),
        //     tuple((Self::parse_property, tag(", "))),
        //     Self::parse_property
        // ))(input)?;

        // // Ok((input, Properties::new(capacity.1, durability.1, flavor.1, texture.1, calories.1)));
        // // Ok((input, Properties::from(capacity.1, durability.1, flavor.1, texture.1, calories.1)));
        Ok((input, Properties::from(props)))

        // todo!()   
    }

    // Parse a single property like "capacity 2"
    fn parse_property(input: &str) -> IResult<&str, (&str, i32)> {
        // dbg!(input);

        tuple((
            // Property name
            take_until(" "),
            // Space followed by number
            preceded(space1, i32)
        ))(input)
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        dbg!(&self);

        todo!()
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-15.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1); 
    // println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let mut day = Day::from_str(input).unwrap();
        let result = day.solve(Part::One).unwrap();

        assert_eq!(result, "62842880");
    }


}