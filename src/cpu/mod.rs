//! Represents the CPU
//! This is the main component of the emulator
//! It contains the following:
//! 1. Registers
//! 2. Data memory
//! 3. Instruction memory
//! 4. Program counter

pub mod instructions;
pub mod memory;
pub mod registers;

use memory::Memory;
use instructions::{Instruction, Opcode};
use registers::{Register, PC, MDR, CIR, ACC};

/// Represents the CPU
pub struct CPU {
    /// Registers
    pc: PC,
    mdr: MDR,
    cir: CIR,
    acc: ACC,
    data_memory: Memory,
    instruction_memory: Memory,

    /// Flag to indicate if the CPU is running
    running: bool,
}

impl CPU {
    /// Initialise a new CPU
    pub fn new(data_memory_size: u32, instruction_memory_size: u32) -> CPU {
        CPU {
            pc: PC::new(),
            mdr: MDR::new(),
            cir: CIR::new(),
            acc: ACC::new(),
            data_memory: Memory::new(data_memory_size),
            instruction_memory: Memory::new(instruction_memory_size),
            running: false,
        }
    }

    /// Load a program into the instruction memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            self.instruction_memory.write(i as u32, *byte);
        }
    }

    /// Start the CPU
    pub fn start(&mut self) {
        self.running = true;
        while self.running {
            self.fetch();
            self.decode();
            self.execute();
        }
    }

    /// Fetch the next instruction
    fn fetch(&mut self) {
        // Get the address of the next instruction
        let address = self.pc.get(); // MAR
        
        // Read the instruction from the instruction memory
        let instruction = self.instruction_memory.read(address as u32); // MDR
  
        // Increment the program counter
        self.pc.set(address + 1);

        // Set the MDR to the instruction
        self.mdr.set(instruction);
    }

    /// Decode the current instruction
    fn decode(&mut self) {
        // Get the instruction from the MDR
        let instruction = self.mdr.get();

        // Decode the instruction and set to CIR
        // Decoding handled by CIR
        self.cir.set(instruction);
    }

    /// Execute the current instruction
    fn execute(&mut self) {
        unimplemented!()
    }
}