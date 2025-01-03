//! Day 16: Aunt Sue

use std::str::FromStr;

use aoc_ornaments::{Part, Solution};
use nom::{bytes::complete::{tag, take_until}, character::complete::{space0, space1, u32}, multi::separated_list1, sequence::{delimited, preceded, tuple}, IResult};

#[derive(Debug, PartialEq, Eq)]
struct Sue {
    children: u32,
    cats: u32,
    samoyeds: u32,
    pomeranians: u32,
    akitas: u32,
    vizslas: u32,
    goldfish: u32,
    trees: u32,
    cars: u32,
    perfumes: u32,
}

impl FromStr for Sue {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        // let sue = SueBuilder::from(Day::parse_properties(input)?);
        // Ok(sue.make_sue())

        Ok(SueBuilder::from(input.lines().map(|line| {
            // let (_, sue) = Day::parse_sue(line).unwrap();
            let (_, prop) = Day::parse_property(line).expect("valid input");

            // dbg!(prop)
            prop
        }).collect::<Vec<(&str, u32)>>()).make_sue())

        // dbg!(Day::parse_properties(input).expect("ok"));

        // todo!()
    }
}

enum SueProperties {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl FromStr for SueProperties {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        match input.to_lowercase().as_str() {
            "children" => Ok(Self::Children),
            "cats" => Ok(Self::Cats),
            "samoyeds" => Ok(Self::Samoyeds),
            "pomeranians" => Ok(Self::Pomeranians),
            "akitas" => Ok(Self::Akitas),
            "vizslas" => Ok(Self::Vizslas),
            "goldfish" => Ok(Self::Goldfish),
            "trees" => Ok(Self::Trees),
            "cars" => Ok(Self::Cars),
            "perfumes" => Ok(Self::Perfumes),
            _ => unimplemented!("invalid input")
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SueBuilder {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl SueBuilder {
    fn new() -> Self {
        Self {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn make_sue(self) -> Sue {
        Sue {
            children: self.children.unwrap_or(0),
            cats: self.cats.unwrap_or(0),
            samoyeds: self.samoyeds.unwrap_or(0),
            pomeranians: self.pomeranians.unwrap_or(0),
            akitas: self.akitas.unwrap_or(0),
            vizslas: self.vizslas.unwrap_or(0),
            goldfish: self.goldfish.unwrap_or(0),
            trees: self.trees.unwrap_or(0),
            cars: self.cars.unwrap_or(0),
            perfumes: self.perfumes.unwrap_or(0),
        }
    }

    fn set_value(&mut self, key: SueProperties, value: u32) {
        match key {
            SueProperties::Children => self.children = Some(value),
            SueProperties::Cats => self.cats = Some(value),
            SueProperties::Samoyeds => self.samoyeds = Some(value),
            SueProperties::Pomeranians => self.pomeranians = Some(value),
            SueProperties::Akitas => self.akitas = Some(value),
            SueProperties::Vizslas => self.vizslas = Some(value),
            SueProperties::Goldfish => self.goldfish = Some(value),
            SueProperties::Trees => self.trees = Some(value),
            SueProperties::Cars => self.cars = Some(value),
            SueProperties::Perfumes => self.perfumes = Some(value),
        }
    }
}

impl From<Vec<(&str, u32)>> for SueBuilder {
    fn from(props: Vec<(&str, u32)>) -> Self {
        // dbg!(&props);

        let mut sue = SueBuilder::new();

        props.iter().for_each(|(key, value)| {
            // match key {
            //     &"children" => sue.set_value(SueProperties::Children, *value),
            //     _ => todo!(),
            // }
            sue.set_value(SueProperties::from_str(key).unwrap(), *value);
        });

        sue
    }
}

impl IntoIterator for SueBuilder {
    type Item = (SueProperties, Option<u32>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        vec![
            (SueProperties::Children, self.children.take()),
            (SueProperties::Cats, self.cats.take()),
            (SueProperties::Samoyeds, self.samoyeds.take()),
            (SueProperties::Pomeranians, self.pomeranians.take()),
            (SueProperties::Akitas, self.akitas.take()),
            (SueProperties::Vizslas, self.vizslas.take()),
            (SueProperties::Goldfish, self.goldfish.take()),
            (SueProperties::Trees, self.trees.take()),
            (SueProperties::Cars, self.cars.take()),
            (SueProperties::Perfumes, self.perfumes.take()),
        ].into_iter()
    }
}

impl PartialEq<Sue> for SueBuilder {
    fn eq(&self, other: &Sue) -> bool {

        self.into_iter().filter_map(|item| {
            if item.1.is_some() {
                Some(item)
            } else {
                None
            }
        }).all(|(key, value)| {
            let value = value.unwrap();

            match key {
                SueProperties::Children => value == other.children,
                SueProperties::Cats => value == other.cats,
                SueProperties::Samoyeds => value == other.samoyeds,
                SueProperties::Pomeranians => value == other.pomeranians,
                SueProperties::Akitas => value == other.akitas,
                SueProperties::Vizslas => value == other.vizslas,
                SueProperties::Goldfish => value == other.goldfish,
                SueProperties::Trees => value == other.trees,
                SueProperties::Cars => value == other.cars,
                SueProperties::Perfumes => value == other.perfumes,
            }
        })
    }
}

/// a vec because we know the input is already sorted
#[derive(Debug)]
struct Day(Vec<SueBuilder>);

impl std::ops::Deref for Day {
    type Target = Vec<SueBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        // inefficient for tests, but valid for real input
        // let sues = Vec::with_capacity(500);

        Ok(Self(input.lines().map(|line| {
            let (_, sue) = Day::parse_sue(line).unwrap();
            // dbg!(sue)
            sue
        }).collect()))
    }
}

impl Day {
    fn parse_sue(input: &str) -> IResult<&str, SueBuilder> {
        let (list, _) = take_until(": ")(input)?;
        let (list, _) = preceded(tag(":"), space1)(list)?;
        // dbg!(list);

        let (remainder, props) = Self::parse_properties(list)?;
        // dbg!(props);

        Ok((remainder, SueBuilder::from(props)))
    }

    fn parse_properties(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
        // dbg!(input);

        // let (remainder, props) = 
        separated_list1(
            delimited(space0, tag(","), space1),  // This handles " , " as separator
            Self::parse_property
        )(input)

        // dbg!(remainder, props);

        // todo!()
        // Ok((remainder, SueBuilder::from(props)))
    }

    /// Parse a single property like "cars: 2"
    fn parse_property(input: &str) -> IResult<&str, (&str, u32)> {
        // dbg!(input);

        tuple((
            // Property name
            take_until(": "),
            // colon + Space followed by number
            preceded(tag(": "), u32)
        ))(input)
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        let aunt_sue = Sue::from_str("children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1").expect("ok");

        dbg!(&aunt_sue);

        let sue = self.iter().position(|sue| {
            // dbg!(sue);
            // false
            sue == &aunt_sue
        }).expect("not found"); //.ok_or_else(|| miette::MietteError::from("no solution"));

        Ok(sue + 1)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-16.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}