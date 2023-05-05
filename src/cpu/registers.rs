use super::instructions::{Instruction};

pub trait Register {
    /// Get the value of the register
    fn get(&self) -> u8;
    /// Set the value of the register
    fn set(&mut self, value: u8);
}

/// Program Counter used to store the address of the next instruction
pub struct PC {
    /// Program Counter
    data: u8,
}

/// Memory Data Register used to store the data fetched from the memory
/// location
/// This is also used to store the data to be written to the memory location
/// during a store instruction
pub struct MDR {
    /// Memory Data Register
    data: u8,
}

/// Current Instruction Register used to store the current instruction
/// being executed, split into opcode and operand
pub struct CIR {
    /// Current Instruction Register
    data: Option<Instruction>,
}

/// Accumulator used to store the result of arithmetic and logical operations
pub struct ACC {
    /// Accumulator
    data: u8,
}

impl Register for PC {
    /// Get the value of the register
    fn get(&self) -> u8 {
        self.data
    }
    /// Set the value of the register
    fn set(&mut self, value: u8) {
        self.data = value;
    }
}

impl Register for MDR {
    /// Get the value of the register
    fn get(&self) -> u8 {
        self.data
    }
    /// Set the value of the register
    fn set(&mut self, value: u8) {
        self.data = value;
    }
}

impl Register for CIR {
    /// Get the value of the register
    fn get(&self) -> u8 {
        unimplemented!("CIR::get() not implemented")
    }
    /// Set the value of the register
    fn set(&mut self, value: u8) {
        self.data = Some(Instruction::from_byte(value));
    }
}

impl Register for ACC {
    /// Get the value of the register
    fn get(&self) -> u8 {
        self.data
    }
    /// Set the value of the register
    fn set(&mut self, value: u8) {
        self.data = value;
    }
}

impl PC {
    /// Create a new PC
    pub fn new() -> PC {
        PC {
            data: 0,
        }
    }
}

impl MDR {
    /// Create a new MDR
    pub fn new() -> MDR {
        MDR {
            data: 0,
        }
    }
}

impl CIR {
    /// Create a new CIR
    pub fn new() -> CIR {
        CIR {
            data: None,
        }
    }

    /// Get instruction
    pub fn get_instruction(&self) -> Option<Instruction> {
        self.data.clone()
    }
}

impl ACC {
    /// Create a new ACC
    pub fn new() -> ACC {
        ACC {
            data: 0,
        }
    }
}