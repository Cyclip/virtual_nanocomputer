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
    pub acc: ACC,
    pub data_memory: Memory,
    pub instruction_memory: Memory,

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

        println!("Fetching instruction at address: {} -> {}", address, instruction);
 
        // Increment the program counter by 2 (2 bytes per instruction)
        self.pc.set(address + 2);

        // Set the MDR to the instruction
        self.mdr.set(instruction);
    }

    /// Decode the current instruction
    fn decode(&mut self) {
        // Get the instruction from the MDR
        let instruction = self.mdr.get();

        println!("Decoding instruction: {}", instruction);

        // Decode the instruction and set to CIR
        // Decoding handled by CIR
        self.cir.set(instruction);

        println!("Decoded instruction: {} -> {:?}", instruction, self.cir.get_instruction().unwrap());
    }

    /// Execute the current instruction
    fn execute(&mut self) {
        // Get the instruction from the CIR
        let instruction = self.cir.get_instruction();

        println!("Executing instruction: {:?}", instruction);

        // Execute the instruction
        match instruction {
            None => {}, // ignore
            Some(instr) => {
                // Get the operand from the instruction
                let operand_addr = instr.operand;
                
                match instr.opcode {
                    Opcode::ADD => {
                        // Get the operand from the data memory
                        let operand = self.data_memory.read(operand_addr as u32);

                        // Add the operand to the accumulator
                        let result = self.acc.get() + operand;

                        // Set the accumulator to the result
                        self.acc.set(result);
                    },

                    Opcode::SUB => {
                        // Get the operand from the data memory
                        let operand = self.data_memory.read(operand_addr as u32);

                        // Subtract the operand from the accumulator
                        let result = self.acc.get() - operand;

                        // Set the accumulator to the result
                        self.acc.set(result);
                    },

                    Opcode::MUL => {
                        // Get the operand from the data memory
                        let operand = self.data_memory.read(operand_addr as u32);

                        // Multiply the operand with the accumulator
                        let result = self.acc.get() * operand;

                        // Set the accumulator to the result
                        self.acc.set(result);
                    },

                    Opcode::DIV => {
                        // Get the operand from the data memory
                        let operand = self.data_memory.read(operand_addr as u32);

                        // Divide the accumulator by the operand
                        let result = self.acc.get() / operand;

                        // Set the accumulator to the result
                        self.acc.set(result);
                    },

                    Opcode::STA => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Store the accumulator in the data memory
                        self.data_memory.write(operand_addr as u32, acc);
                    },

                    Opcode::LDA => {
                        // Get the operand from the data memory
                        let operand = self.data_memory.read(operand_addr as u32);

                        // Set the accumulator to the operand
                        self.acc.set(operand);
                    },

                    Opcode::JMP => {
                        // Get the operand from the instruction
                        let operand_addr = instr.operand;

                        // Set the program counter to the operand
                        self.pc.set(operand_addr);
                    },

                    Opcode::JEQ => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is zero
                        if acc == 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::JNE => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is not zero
                        if acc != 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::JGT => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is greater than zero
                        if acc > 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::JLT => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is less than zero
                        if acc < 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::JZ => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is zero
                        if acc == 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::JNZ => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Check if the accumulator is not zero
                        if acc != 0 {
                            // Set the program counter to the operand
                            self.pc.set(operand_addr);
                        }
                    },

                    Opcode::HLT => {
                        // Stop the CPU
                        self.running = false;
                    },

                    Opcode::INP => {
                        // Get the input from the user
                        let input = self.get_input();

                        // Set the accumulator to the input
                        self.acc.set(input);
                    },

                    Opcode::OUT => {
                        // Get the accumulator
                        let acc = self.acc.get();

                        // Output the accumulator
                        self.output(acc);
                    },

                    Opcode::DAT => {
                        // Get the operand from the instruction
                        let operand = instr.operand;

                        // Store the operand in the data memory
                        self.data_memory.write(operand_addr as u32, operand);
                    },
                }
            }
        }
    }

    /// Get input from the user
    fn get_input(&mut self) -> u8 {
        unimplemented!()
    }

    /// Output data
    fn output(&mut self, data: u8) {
        unimplemented!()
    }
}