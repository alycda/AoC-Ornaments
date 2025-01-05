pub trait Machine {
    type Register: Copy;
    // type Registers: std::collections::HashMap<String, Self::Register>;
    // type Instruction: std::str::FromStr;
    
    fn get_register(&self, reg: &str) -> Self::Register;
    fn set_register(&mut self, reg: &str, value: Self::Register);
    fn execute_instruction(&mut self) -> Option<usize>;
}