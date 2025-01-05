//! Day 23: Opening the Turing Lock

use std::str::FromStr;

use aoc_ornaments::{nom::split_newlines, Part, Solution};
use nom::{branch::alt, bytes::complete::tag, character::{complete::{alpha1, digit1, not_line_ending, space0, space1}, streaming::line_ending}, combinator::{map, opt}, error::{VerboseError, VerboseErrorKind}, sequence::tuple, IResult};

#[derive(Debug)]
struct Day {
    register_a: usize,
    register_b: usize,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    /// hlf r sets register r to half its current value, then continues with the next instruction.
    Half(char),
    /// tpl r sets register r to triple its current value, then continues with the next instruction.
    Triple(char),
    /// inc r increments register r, adding 1 to it, then continues with the next instruction.
    Increment(char),
    /// jmp offset is a jump; it continues with the instruction offset away relative to itself.
    Jump(i32),
    /// jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    JumpIfEven(char, i32),
    /// jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).
    JumpIfOne(char, i32),
}

impl Instruction {
    fn parse_number(input: &str) -> IResult<&str, i32> {
        let (input, (_, sign, number)) = tuple((
            space0, 
            alt((tag("+"), tag("-"))),
            digit1
        ))(input)?;

        let number = format!("{}{}", sign, number).parse().unwrap();
        Ok((input, number))
    }

    fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        let (input, instruction) = alpha1(input)?;

        if let "jmp" = instruction {
            let (input, offset) = Self::parse_number(input)?;
            return Ok((input, Self::Jump(offset)));
        }

        let (input, _) = space1(input)?;
        let (input, register) = alpha1(input)?;

        match instruction {
            "hlf" => Ok((input, Self::Half(register.chars().next().unwrap()))),
            "tpl" => Ok((input, Self::Triple(register.chars().next().unwrap()))),
            "inc" => Ok((input, Self::Increment(register.chars().next().unwrap()))),
            "jie" => {
                let (input, _) = tuple((tag(","), space0))(input)?;
                let (input, offset) = Self::parse_number(input)?;
                Ok((input, Self::JumpIfEven(register.chars().next().unwrap(), offset)))
            },
            "jio" => {
                let (input, _) = tuple((tag(","), space0))(input)?;
                let (input, offset) = Self::parse_number(input)?;
                Ok((input, Self::JumpIfOne(register.chars().next().unwrap(), offset)))
            },
            _ => todo!(),
        }
    }

    fn parse_3(input: &str) -> IResult<&str, ()> {
        todo!()
    }
}

impl FromStr for Instruction {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        // dbg!(input);

        let (_, instruction) = Instruction::parse_instruction(input)
            .map_err(|e| miette::miette!(e.to_owned()))?;

        Ok(instruction)

        // let (_, b) = tuple((alpha1, space1, 
        //     alt((alpha1, tuple((opt(tag("+"), tag("-")), digit1)))), 
        //     opt(tuple((opt(","), space0, alt((tag("+"), tag("-"))), digit1))) ))
        //     (input)
        //         .map_err(|e| miette::miette!(e.to_owned()))?;

        // let (_, b) = tuple((
        //     alpha1,                 // instruction
        //     space1,                 // required space
        //     alt((
        //         // First case: register with optional offset (jio a, +18)
        //         tuple((
        //             alpha1,        // register
        //             opt(tuple((    // optional number part
        //                 opt(tag(",")),
        //                 space0,
        //                 alt((tag("+"), tag("-"))),
        //                 digit1
        //             )))
        //         )),
        //         // Second case: just number (jmp +22)
        //         tuple((
        //             alt((tag("+"), tag("-"))),
        //             digit1
        //         ))
        //     ))
        // ))(input).map_err(|e| miette::miette!(e.to_owned()))?;

        // let (_, (instruction, _, (register, extra))) = tuple((
        //     alpha1,                 // instruction
        //     space1,                 // required space
        //     alt((
        //         // First case: register with optional offset (jio a, +18)
        //         map(tuple((
        //             alpha1,        // register
        //             opt(tuple((    // optional number part
        //                 opt(tag(",")),
        //                 space0,
        //                 alt((tag("+"), tag("-"))),
        //                 digit1
        //             )))
        //         )), |(reg, num)| (Some(reg), num)),
        //         // Second case: just number (jmp +22)
        //         map(tuple((
        //             alt((tag("+"), tag("-"))),
        //             digit1
        //         )), |(sign, num)| (None, Some((None, "", sign, num))))
        //     ))
        // ))(input).map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;

        // dbg!(instruction, register.unwrap(), extra);

        // let (input, (instruction, _, register)) = tuple((alpha1, space1, alpha1))(input)
        //     .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;

        // // dbg!(instruction, register);

        // dbg!(input);
        // let (_, (_, _, sign, number)) = tuple::<_, _, VerboseError<&str>, _>((opt(tag(",")), space0, alt((tag("+"), tag("-"))), digit1))(input)
        //     // .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;
        //     .map_err(|e| match e {
        //         nom::Err::Error(e) | nom::Err::Failure(e) => {
        //             nom::Err::Error(VerboseError { errors: vec![(input, VerboseErrorKind::Context("parsing number with sign"))] })
        //         },
        //         nom::Err::Incomplete(n) => nom::Err::Incomplete(n),
        //     }).unwrap();
        // // dbg!(sign, number);

        // let (a, b) = opt(alt((line_ending, not_line_ending)))(input)
        //     .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;

        // let (a, b) = opt(alt((line_ending, tuple((opt(tag(",")), space0, alt((tag("+"), tag("-")), digit1))))))(input)
        //     .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;


        // let (input, offset) = opt(tuple((alt((tag("+"), tag("-"))), digit1)))(input)
        //     .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;




        // let (a, b) = opt(alt((line_ending, tuple((not_line_ending, alt((tag("+"), tag("-"))), digit1)))))(input).expect("failed to parse");
            // .map_err(|e: nom::Err<(&str, nom::error::ErrorKind)>| miette::miette!(e.to_owned()))?;
        // let (a, b) = opt((line_ending, tuple((not_line_ending, alt((tag("+"), tag("-"))), digit1)))(input);

        // let parts: Vec<&str> = input.split_whitespace().collect();

        // match parts.as_slice() {
        //     ["hlf", reg] => Ok(Self::Half(reg.chars().next().unwrap())),
        //     ["tpl", reg] => Ok(Self::Triple(reg.chars().next().unwrap())),
        //     ["inc", reg] => Ok(Self::Increment(reg.chars().next().unwrap())),
        //     ["jmp", offset] => Ok(Self::Jump(offset.parse()?)),
        //     ["jie", reg, offset] => Ok(Self::JumpIfEven(reg.chars().next().unwrap(), offset.parse()?)),
        //     ["jio", reg, offset] => Ok(Self::JumpIfOne(reg.chars().next().unwrap(), offset.parse()?)),
        //     _ => Err(miette::MietteError::from("invalid input")),
        // }

        // todo!();

        // match instruction {
        //     "jio" => {
        //         let (_, _, sign, number) = extra.unwrap();
        //         Ok(Self::JumpIfOne(register.unwrap().chars().next().unwrap(), format!("{}{}", sign, number).parse().unwrap()))
        //     },
        //     "inc" => Ok(Self::Increment(register.unwrap().chars().next().unwrap())),
        //     "tpl" => Ok(Self::Triple(register.unwrap().chars().next().unwrap())),
        //     "jmp" => {
        //         let (_, _, sign, number) = extra.unwrap();
        //         Ok(Self::Jump(format!("{}{}", sign, number).parse().unwrap()))
        //     },
        //     _ => todo!(),
        // }
    }
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, lines) = split_newlines(input)?;

        let instructions = lines.iter().map(|line| {
            Instruction::from_str(line)
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(instructions))
    }
}

impl Day {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            instructions,
        }
    }

    fn execute(&mut self) {
        todo!()
    }
}

impl Solution for Day {
    type Output = usize;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        dbg!(&self);

        self.execute();

        Ok(self.register_b)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-23.txt"))?;
    let part1 = day.solve(Part::One)?;
    // let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);

    Ok(())
}