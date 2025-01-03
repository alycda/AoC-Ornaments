//! Day 7: Some Assembly Required

use std::{collections::HashMap, str::FromStr};

use aoc_ornaments::{bits::LogicGate, Part, Solution};

type Wires = HashMap<String, String>;
type Instructions = Vec<LogicGate<Operand>>;

#[derive(Debug, Clone, Copy)]
enum Operand {
    /// bitwise and
    And,
    RightShift,
    /// bitwise or
    Or,
    LeftShift,
    /// bitwise complement
    Not
}

impl FromStr for Operand {
    type Err = miette::Error;

    fn from_str(s: &str) -> miette::Result<Self> {
        match s {
            "AND" => Ok(Self::And),
            "RSHIFT" => Ok(Self::RightShift),
            "OR" => Ok(Self::Or),
            "LSHIFT" => Ok(Self::LeftShift),
            "NOT" => Ok(Self::Not),
            _ => panic!("Invalid operand: {s}")
            // _ => Err(miette::MietteError::from("Invalid operand").into()),
        }
    }
}

#[derive(Debug)]
struct Day {
    wires: Wires,
    instructions: Instructions,
}

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut wires = HashMap::new();

        let instructions = input.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();

                match parts.as_slice() {
                    // signal is provided to wire (initialization)
                    [signal, "->", wire] => {
                        wires.insert(wire.to_string(), signal.to_string());

                        None
                    }
                    // bitwise complement
                    ["NOT", input, "->", output] => {
                        // let input = input.parse().unwrap();
                        // let output = output.parse().unwrap();

                        // Some(dbg!(input.to_owned(), output.to_owned(), "", ""))

                        Some(LogicGate::new(input.to_string(), "0".to_owned(),  Operand::Not, output.to_string()))
                    }
                    // Logic Gates
                    [left, op, right, "->", output] => {
                        // let left = left.parse().unwrap();
                        let operation = op.parse().unwrap();
                        // let right = right.parse().unwrap();
                        // let output = output.parse().unwrap();

                        // dbg!(left, op, right, output);

                        Some(LogicGate::new(left.to_string(), right.to_string(), operation, output.to_string()))

                        // Some((left.to_owned(), op.to_owned(), right.to_owned(), output.to_owned()))
                    }
                    _ => panic!("Invalid instruction")
                }

            }).collect();

        // dbg!(&wires, &instructions);

        Ok(Self { wires, instructions })
    }
}

impl Day {
    fn do_instructions(&mut self) -> miette::Result<()> {
        let mut pending = self.instructions.clone();
        let mut progress = true;
        let mut evaluated_wires: HashMap<String, u16> = HashMap::new();

        // First pass - convert any pure number strings to u16
        for (wire, value) in &self.wires {
            if let Ok(num) = value.parse::<u16>() {
                evaluated_wires.insert(wire.clone(), num);
            }
        }

        while progress && !pending.is_empty() {
            progress = false;
            
            let (ready, still_pending): (Vec<_>, Vec<_>) = pending.into_iter()
                .partition(|gate| {
                    // Check if we can evaluate this gate
                    match gate.operation {
                        Operand::Not => {
                            // For NOT, we only need the left operand
                            is_evaluatable(&gate.left, &evaluated_wires, &self.wires)
                        },
                        Operand::RightShift => {
                            // For RightShift, we need the left operand (right is a number)
                            is_evaluatable(&gate.left, &evaluated_wires, &self.wires)
                        },
                        _ => {
                            // For other operations, we need both operand
                            is_evaluatable(&gate.left, &evaluated_wires, &self.wires) && 
                            is_evaluatable(&gate.right, &evaluated_wires, &self.wires)
                        }
                    }
                });

            // Helper function to check if we can evaluate a wire
            fn is_evaluatable(wire: &str, evaluated: &HashMap<String, u16>, initial: &HashMap<String, String>) -> bool {
                evaluated.contains_key(wire) || 
                wire.parse::<u16>().is_ok() ||
                initial.get(wire).map_or(false, |v| v.parse::<u16>().is_ok())
            }

            pending = still_pending;

            for gate in ready {
                // dbg!(&gate);

                // let left: u16 = match self.wires[&gate.left].parse() {
                //     Ok(num) => num,
                //     Err(_) => evaluated_wires[&gate.left], // Fallback to already evaluated wires
                // };

                let left: u16 = if let Ok(num) = gate.left.parse::<u16>() {
                    // Direct number in the left operand
                    num
                } else {
                    // Wire reference in the left operand
                    match self.wires[&gate.left].parse() {
                        Ok(num) => num,
                        Err(_) => evaluated_wires[&gate.left], // Fallback to already evaluated wires
                    }
                };

                // let left: u32 = self.wires[&gate.left].parse().unwrap();
                // let right: u16 = self.wires[&gate.right].parse().unwrap();

                let result: u16 = match gate.operation {
                    Operand::Or => {
                        // let left: u16 = self.wires[&gate.left].parse().unwrap();
                        // let right: u16 = self.wires[&gate.right].parse().unwrap();
                        let right: u16 = if let Ok(num) = gate.right.parse::<u16>() {
                            num
                        } else {
                            match self.wires[&gate.right].parse() {
                                Ok(num) => num,
                                Err(_) => evaluated_wires[&gate.right],
                            }
                        };

                        left | right
                    },
                    Operand::And => {
                        // let left: u16 = self.wires[&gate.left].parse().unwrap();
                        // let right: u16 = self.wires[&gate.right].parse().unwrap();
                        let right: u16 = if let Ok(num) = gate.right.parse::<u16>() {
                            num
                        } else {
                            match self.wires[&gate.right].parse() {
                                Ok(num) => num,
                                Err(_) => evaluated_wires[&gate.right],
                            }
                        };
                        
                        left & right
                    },
                    // Bitwise NOT only needs left operand
                    Operand::Not => {
                        // let left: u16 = self.wires[&gate.left].parse().unwrap();

                        !left
                    },  
                    Operand::RightShift => {
                        // let left: u16 = self.wires[&gate.left].parse().unwrap();
                        let shift: u16 = gate.right.parse().unwrap(); // Right operand is direct number

                        left >> shift
                    },
                    Operand::LeftShift => {
                        // let left: u16 = self.wires[&gate.left].parse().unwrap();
                        let shift: u16 = gate.right.parse().unwrap(); // Right operand is direct number

                        left << shift
                    },

                    // _ => todo!(),
                };
                
                self.wires.insert(gate.output, result.to_string());

                progress = true;
            }
        }

        Ok(())
    }

    fn resolve_wire(&self, wire: &str) -> u16 {
        // First try to parse as a direct number
        if let Ok(num) = wire.parse::<u16>() {
            return num;
        }
        
        // Get the value from wires
        let value = self.wires.get(wire).unwrap();
        
        // Try to parse that value as a number
        match value.parse::<u16>() {
            Ok(num) => num,
            // If it's not a number, it must be another wire reference
            Err(_) => self.resolve_wire(value),
        }
    }
}

impl Solution for Day {
    type Output = u16;

    fn part1(&mut self) -> miette::Result<Self::Output> {
        // // dbg!(&self.instructions);

        // let mut pending = self.instructions.clone();
        // let mut progress = true;

        // while progress && !pending.is_empty() {
        //     todo!()
        // }

        self.do_instructions()?;

        // dbg!(&self.wires.get("a"), &self.wires.get("lx"));

        // todo!();

        // Ok(self.wires.get("a").unwrap().parse().unwrap())
        Ok(self.resolve_wire("a"))
    }

    fn part2(&mut self) -> miette::Result<Self::Output> {
        // Store the result from part 1
        let part1_result = self.resolve_wire("a").to_string();
        
        // Reset all wires to initial state
        *self = Day::from_str(include_str!("./inputs/2015-12-07.txt"))?;
        
        // Override wire b with part1's result
        self.wires.insert("b".to_string(), part1_result);
        
        // Run simulation again
        self.do_instructions()?;
        
        // Get new value for wire a
        Ok(self.resolve_wire("a"))
    }
}

fn main() -> miette::Result<()> {
    let mut day = Day::from_str(include_str!("./inputs/2015-12-07.txt"))?;
    let part1 = day.solve(Part::One)?;
    let part2 = day.solve(Part::Two)?;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn format_wires(wires: &Wires) -> String {
        let mut entries: Vec<_> = wires.iter().collect();
        // Sort by key for consistent ordering
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
        
        entries.iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn test_cases_part1() -> miette::Result<()> {
        let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let mut day = Day::from_str(input)?;
        day.do_instructions()?;

        // dbg!(&day.wires);

        // assert_eq!(day.solve(Part::One).unwrap(), expected.to_string());
        assert_eq!("d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456", format_wires(&day.wires));
        Ok(())
    }
}