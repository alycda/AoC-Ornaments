//! Day 23: Opening the Turing Lock

use std::str::FromStr;

use aoc_ornaments::{nom::split_newlines, Part, Solution};
use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, digit1, space0, space1}, sequence::tuple, IResult};

#[derive(Debug)]
struct Day {
    register_a: i32,
    register_b: i32,
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
}

impl FromStr for Instruction {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, instruction) = Instruction::parse_instruction(input)
            .map_err(|e| miette::miette!(e.to_owned()))?;

        Ok(instruction)
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

    fn get_register(&self, register: char) -> i32 {
        match register {
            'a' => self.register_a,
            'b' => self.register_b,
            _ => panic!("Invalid register"),
        }
    }

    fn set_register(&mut self, register: char, value: i32) {
        match register {
            'a' => self.register_a = value,
            'b' => self.register_b = value,
            _ => panic!("Invalid register"),
        }
    }

    fn execute(&mut self) {
        let mut ip = 0;  // instruction pointer
        
        while ip < self.instructions.len() {
            let instruction = &self.instructions[ip];
            
            // Default is to move to next instruction
            let mut next_ip = ip + 1;
    
            match instruction {
                Instruction::Half(reg) => {
                    self.set_register(*reg, self.get_register(*reg) / 2);
                }
                Instruction::Triple(reg) => {
                    self.set_register(*reg, self.get_register(*reg) * 3);
                }
                Instruction::Increment(reg) => {
                    self.set_register(*reg, self.get_register(*reg) + 1);
                }
                Instruction::Jump(offset) => {
                    next_ip = (ip as i32 + offset) as usize;
                }
                Instruction::JumpIfEven(reg, offset) => {
                    if self.get_register(*reg) % 2 == 0 {
                        next_ip = ip + *offset as usize;
                    }
                }
                Instruction::JumpIfOne(reg, offset) => {
                    if self.get_register(*reg) == 1 {
                        next_ip = ip + *offset as usize;
                    }
                }
            }
    
            ip = next_ip;
        }
    }
}

impl Solution for Day {
    type Output = i32;

    fn part1(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.execute();

        Ok(self.register_b)
    }

    fn part2(&mut self) -> aoc_ornaments::SolutionResult<<Self as Solution>::Output> {
        self.register_a = 1;
        self.execute();

        Ok(self.register_b)
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-23.txt"))?;
    let part1 = day.solve(Part::One)?;
    // state is dirty after part1, need to reset
    let mut day = Day::from_str(include_str!("./inputs/2015-12-23.txt"))?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}