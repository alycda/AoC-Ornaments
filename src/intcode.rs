use std::collections::HashMap;

pub trait Machine {
    type Register: Copy;
    // type Registers: std::collections::HashMap<String, Self::Register>;
    // type Instruction: std::str::FromStr;
    
    fn get_register(&self, reg: &str) -> Self::Register;
    fn set_register(&mut self, reg: &str, value: Self::Register);
    fn execute_instruction(&mut self) -> Option<usize>;
}

#[derive(Debug)]
pub struct VirtualMachine<I, T: Copy> {
    registers: HashMap<char, T>,
    pub instructions: Vec<I>,
    /// Instruction Pointer
    _ip: usize,
}

impl<I, T: Copy> VirtualMachine<I, T> {
    pub fn new(registers: HashMap<char, T>, instructions: Vec<I>) -> Self {
        Self {
            registers,
            instructions,
            _ip: 0,
        }
    }

    pub fn get_register(&self, reg: &char) -> T {
        *self.registers.get(reg).unwrap()
    }

    pub fn set_register(&mut self, reg: char, value: T) {
        self.registers.insert(reg, value);
    }
}