use std::{collections::BTreeMap, str::FromStr};

pub type Wires<T> = BTreeMap<String, T>;
pub type Instructions<O> = Vec<LogicGate<O>>;

#[derive(Debug)]
pub struct LogicCircuit<T, O: Clone> {
    pub wires: BTreeMap<String, T>,
    instructions: Vec<LogicGate<O>>,
    evaluated: BTreeMap<String, u16>,  // Cache for evaluated results
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    And,
    RightShift,
    Or,
    LeftShift,
    Not,
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
            _ => Err(miette::miette!("Invalid operand: {s}")),
        }
    }
}

impl<O> LogicCircuit<String, O> 
where 
    O: Clone
{
    pub fn new(wires: BTreeMap<String, String>, instructions: Vec<LogicGate<O>>) -> Self {
        Self {
            wires,
            instructions,
            evaluated: BTreeMap::new(),
        }
    }

    fn get_wire_value(&self, wire: &str) -> Option<u16> {
        // First check if we have already evaluated this wire
        if let Some(&value) = self.evaluated.get(wire) {
            return Some(value);
        }

        // Try parsing as direct number
        if let Ok(num) = wire.parse::<u16>() {
            return Some(num);
        }

        // Check if it's in initial wires and can be parsed as number
        if let Some(value) = self.wires.get(wire) {
            if let Ok(num) = value.parse::<u16>() {
                return Some(num);
            }

            // If not a number, recursively resolve the referenced wire
            return self.get_wire_value(value);
        }

        None
    }
}

impl LogicCircuit<String, Operand> {
    pub fn execute(&mut self) -> miette::Result<()> {
        let mut pending = self.instructions.clone();
        let mut progress = true;

        // First pass - convert any pure number strings to u16
        for (wire, value) in &self.wires {
            if let Ok(num) = value.parse::<u16>() {
                self.evaluated.insert(wire.clone(), num);
            }
        }

        while progress && !pending.is_empty() {
            progress = false;
            
            let (ready, still_pending): (Vec<_>, Vec<_>) = pending.into_iter()
                .partition(|gate| {
                    match gate.operation {
                        Operand::Not => self.get_wire_value(&gate.left).is_some(),
                        Operand::RightShift | Operand::LeftShift => 
                            self.get_wire_value(&gate.left).is_some() && gate.right.parse::<u16>().is_ok(),
                        _ => self.get_wire_value(&gate.left).is_some() && 
                             self.get_wire_value(&gate.right).is_some()
                    }
                });

            pending = still_pending;

            for gate in ready {
                let left = self.get_wire_value(&gate.left).unwrap();
                
                let result = match gate.operation {
                    Operand::Or => {
                        let right = self.get_wire_value(&gate.right).unwrap();
                        left | right
                    },
                    Operand::And => {
                        let right = self.get_wire_value(&gate.right).unwrap();
                        left & right
                    },
                    Operand::Not => !left,
                    Operand::RightShift => {
                        let shift = gate.right.parse::<u16>().unwrap();
                        left >> shift
                    },
                    Operand::LeftShift => {
                        let shift = gate.right.parse::<u16>().unwrap();
                        left << shift
                    },
                };
                
                self.evaluated.insert(gate.output, result);
                progress = true;
            }
        }

        Ok(())
    }

    pub fn resolve_wire(&self, wire: &str) -> miette::Result<u16> {
        self.get_wire_value(wire)
            .ok_or_else(|| miette::miette!("Unable to resolve wire: {wire}"))
    }
}


// impl<T: Clone, O: Clone> LogicCircuit<T, O> {
//     fn evaluate(mut self) -> miette::Result<Self> {
//         let mut made_progress = true;
//         let mut pending_gates = self.instructions.clone();

//         while made_progress && !pending_gates.is_empty() {
//             made_progress = false;

//             let (ready, still_pending): (Vec<_>, Vec<_>) = pending_gates.drain(..)
//                 .partition(|gate| {
//                     self.wires.contains_key(&gate.left) && 
//                     (!gate.operation.requires_right_operand() || 
//                      gate.right.as_ref().map_or(true, |r| self.wires.contains_key(r)))
//                 });

//             pending_gates = still_pending;

//             for gate in ready {
//                 let left = self.wires[&gate.left].clone();
//                 let right = gate.right.map(|r| self.wires[&r].clone());
                
//                 let result = gate.operation.evaluate(left, right);
//                 self.wires.insert(gate.output, result);

//                 made_progress = true;
//             }
//         }

//         Ok(self)
//     }
// }

#[derive(Debug, Clone)]
pub struct LogicGate<O: Clone> {
    pub left: String,
    pub right: String,
    pub operation: O,
    pub output: String,
}

impl<O: Clone> LogicGate<O> {
    pub fn new(left: String, right: String, operation: O, output: String) -> Self {
        Self { left, right, operation, output }
    }
}