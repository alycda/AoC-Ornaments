use std::hash::Hash;
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
pub struct VirtualMachine<I, K, T: Clone> 
where 
    K: Hash + Eq + Into<String>, 
{
    pub registers: HashMap<K, T>,
    pub instructions: Vec<I>,
    /// Instruction Pointer
    _ip: usize,
}

impl<I, K, T: Clone> VirtualMachine<I, K, T> 
where 
    K: Hash + Eq + Into<String>,
{
    pub fn new(registers: HashMap<K, T>, instructions: Vec<I>) -> Self {
        Self {
            registers,
            instructions,
            _ip: 0,
        }
    }

    pub fn get_register(&self, reg: &K) -> T {
        self.registers.get(reg).unwrap().clone()
    }

    pub fn set_register(&mut self, reg: K, value: T) {
        self.registers.insert(reg, value);
    }
}