//! Day 7: Some Assembly Required

use std::str::FromStr;
use aoc_ornaments::{bits::{LogicCircuit, LogicGate, Operand, Wires}, Part, Solution};
use miette::Context;

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
struct Day(LogicCircuit<String, Operand>);

impl FromStr for Day {
    type Err = miette::Error;

    fn from_str(input: &str) -> miette::Result<Self> {
        let mut wires = Wires::<String>::new();
        let mut instructions = Vec::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                [signal, "->", wire] => {
                    wires.insert(wire.to_string(), signal.to_string());
                }
                ["NOT", input, "->", output] => {
                    instructions.push(LogicGate::new(
                        input.to_string(),
                        "0".to_owned(),
                        Operand::Not,
                        output.to_string(),
                    ));
                }
                [left, op, right, "->", output] => {
                    let operation = op.parse().with_context(|| format!("Invalid operation: {op}"))?;
                    instructions.push(LogicGate::new(
                        left.to_string(),
                        right.to_string(),
                        operation,
                        output.to_string(),
                    ));
                }
                _ => return Err(miette::miette!("Invalid instruction: {}", line)),
            }
        }

        Ok(Self(LogicCircuit::new(wires, instructions)))
    }
}

impl Solution for Day {
    type Output = u16;

    fn solve(&mut self, part: Part) -> miette::Result<String> {
        self.execute()?;

        if let Part::One = part {
            return self.resolve_wire("a").map(|v| v.to_string());
        }

        let part1_result = self.resolve_wire("a")?.to_string();
        // Reset circuit to initial state
        *self = Day::from_str(include_str!("./inputs/2015-12-07.txt"))?;

        // Override wire b with part1's result
        self.wires.insert("b".to_string(), part1_result);
        
        self.execute()?;
        self.resolve_wire("a").map(|v| v.to_string())
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

    #[test]
    fn test_basic_circuit() -> miette::Result<()> {
        let input = "\
            123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";

        let mut circuit = input.parse::<Day>()?;
        circuit.execute()?;

        assert_eq!(circuit.resolve_wire("d")?, 72);
        assert_eq!(circuit.resolve_wire("e")?, 507);
        assert_eq!(circuit.resolve_wire("f")?, 492);
        assert_eq!(circuit.resolve_wire("g")?, 114);
        assert_eq!(circuit.resolve_wire("h")?, 65412);
        assert_eq!(circuit.resolve_wire("i")?, 65079);
        assert_eq!(circuit.resolve_wire("x")?, 123);
        assert_eq!(circuit.resolve_wire("y")?, 456);

        Ok(())
    }
}