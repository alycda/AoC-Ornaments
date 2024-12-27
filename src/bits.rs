use std::collections::BTreeMap;

pub type Wires<T> = BTreeMap<String, T>;
pub type Instructions<O> = Vec<LogicGate<O>>;

/// TODO
struct _LogicCircuit<T, O: Clone> {
    wires: Wires<T>,
    instructions: Instructions<O>,
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