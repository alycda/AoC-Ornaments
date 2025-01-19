//! Day 23: Opening the Turing Lock

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{intcode::VirtualMachine, nom::split_newlines, Part, Solution};
use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, digit1, space0, space1}, sequence::tuple, IResult};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
struct Day(VirtualMachine<Instruction, char, i32>);

#[derive(Debug, Clone, Copy)]
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

impl FromStr for Instruction {
    type Err = miette::Error;

    /// Parse a list of instructions from a string
    fn from_str(input: &str) -> miette::Result<Self> {
        let (_, instruction) = Instruction::parse_instruction(input)
            .map_err(|e| miette::miette!(e.to_owned()))?;

        Ok(instruction)
    }
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
            _ => unimplemented!("invalid instruction"),
        }
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
        let mut registers = HashMap::new();
        registers.insert('a', 0);
        registers.insert('b', 0);

        Self(VirtualMachine::new(registers, instructions))
    }

    fn execute(&mut self) {
        let mut ip = 0;
        
        while ip < self.instructions.len() {
            let instruction = &self.instructions[ip].clone();
            
            // Default is to move to next instruction
            let mut next_ip = ip + 1;
    
            match instruction {
                Instruction::Half(reg) => {
                    let value = self.get_register(reg);
                    self.set_register(*reg, value / 2);
                }
                Instruction::Triple(reg) => {
                    let value = self.get_register(reg);
                    self.set_register(*reg, value * 3);
                }
                Instruction::Increment(reg) => {
                    let value = self.get_register(reg);
                    self.set_register(*reg, value + 1);
                }
                Instruction::Jump(offset) => {
                    next_ip = (ip as i32 + offset) as usize;
                }
                Instruction::JumpIfEven(reg, offset) => {
                    if self.get_register(reg) % 2 == 0 {
                        next_ip = ip + *offset as usize;
                    }
                }
                Instruction::JumpIfOne(reg, offset) => {
                    if self.get_register(reg) == 1 {
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

    /// What is the value in register b when the program is finished executing?
    fn part1(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        self.execute();

        Ok(self.get_register(&'b'))
    }

    /// What is the value in register b after the program has run for a while with register a set to 1?
    fn part2(&mut self) -> aoc_ornaments::SolutionResult<Self::Output> {
        self.set_register('a', 1);
        self.execute();

        Ok(self.get_register(&'b'))
    }
}

/// Run Part 1 and Part 2. Reset the state after Part 1.
fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("../inputs/2015-12-23.txt"))?;
    let part1 = day.solve(Part::One)?;
    // state is dirty after part1, need to reset
    let mut day = Day::from_str(include_str!("../inputs/2015-12-23.txt"))?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution() -> miette::Result<()> {
        let mut day = Day::from_str("inc a
jio a, +2
tpl a
inc a")?;
        day.execute();

        assert_eq!(day.get_register(&'a'), 2);

        Ok(())
    }
}