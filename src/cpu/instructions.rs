//! Module for the instruction set
//! Instructions consist of
//!  Opcode     - 8 bits
//!  Operand    - 8 bits 
//! 
//! Opcode stores the instruction type
//! Operand stores either
//! 1. Memory address (label pointer or raw address)
//! 2. Immediate value (raw value like 3)
//! 
//! 0x01 and onwards are all instructions
//! 0x00 denotes data

/// Instruction struct
#[derive(Clone, Debug)]
pub struct Instruction {
    /// Opcode
    pub opcode: Opcode,
    /// Operand (memory address)
    pub operand: u8,
}

/// All opcodes supported
#[derive(Clone, Debug)]
pub enum Opcode {
    ADD, // Add
    SUB, // Subtract
    MUL, // Multiply
    DIV, // Divide
    STA, // Store Accumulator
    LDA, // Load Accumulator
    JMP, // Jump
    JEQ, // Jump if equal
    JNE, // Jump if not equal
    JGT, // Jump if greater than
    JLT, // Jump if less than
    JZ, // Jump if zero
    JNZ, // Jump if not zero
    HLT, // Halt
    INP, // Input
    OUT, // Output
    DAT, // Data
}

impl Instruction {
    /// Create a new instruction
    pub fn new(opcode: Opcode, operand: u8) -> Instruction {
        Instruction {
            opcode,
            operand,
        }
    }

    /// Create a new instruction from raw byte
    pub fn from_byte(byte: u8) -> Instruction {
        let opcode = Opcode::from_byte(byte & 0b0000_1111);
        let operand = byte & 0b1111_0000;
        Instruction::new(opcode, operand)
    }

    /// Get binary representation of instruction
    pub fn to_bin(&self) -> Vec<u8> {
        let mut bin = Vec::new();
        bin.push(self.opcode.to_bin());
        bin.push(self.operand);
        bin
    }
}

impl Opcode {
    /// Get the opcode from a byte
    pub fn from_byte(byte: u8) -> Opcode {
        println!("convering {} to bytecode", byte);
        match byte {
            0x01 => Opcode::ADD, // 0000 0001 or 1
            0x02 => Opcode::SUB, // 0000 0010 or 2
            0x03 => Opcode::MUL, // 0000 0011 or 3
            0x04 => Opcode::DIV, // 0000 0100 or 4
            0x05 => Opcode::STA, // 0000 0101 or 5
            0x06 => Opcode::LDA, // 0000 0110 or 6
            0x07 => Opcode::JMP, // 0000 0111 or 7
            0x08 => Opcode::JEQ, // 0000 1000 or 8
            0x09 => Opcode::JNE, // 0000 1001 or 9
            0x0A => Opcode::JGT, // 0000 1010 or 10
            0x0B => Opcode::JLT, // 0000 1011 or 11
            0x0C => Opcode::JZ,  // 0000 1100 or 12
            0x0D => Opcode::JNZ, // 0000 1101 or 13
            0x0E => Opcode::HLT, // 0000 1110 or 14
            0x0F => Opcode::INP, // 0000 1111 or 15
            0x10 => Opcode::OUT, // 0001 0000 or 16
            0x11 => Opcode::DAT, // 0001 0001 or 17
            _ => panic!("Invalid opcode"),
        }
    }

    /// Get binary representation of opcode
    pub fn to_bin(&self) -> u8 {
        match self {
            Opcode::ADD => 0b0000_0001,
            Opcode::SUB => 0b0000_0010,
            Opcode::MUL => 0b0000_0011,
            Opcode::DIV => 0b0000_0100,
            Opcode::STA => 0b0000_0101,
            Opcode::LDA => 0b0000_0110,
            Opcode::JMP => 0b0000_0111,
            Opcode::JEQ => 0b0000_1000,
            Opcode::JNE => 0b0000_1001,
            Opcode::JGT => 0b0000_1010,
            Opcode::JLT => 0b0000_1011,
            Opcode::JZ =>  0b0000_1100,
            Opcode::JNZ => 0b0000_1101,
            Opcode::HLT => 0b0000_1110,
            Opcode::INP => 0b0000_1111,
            Opcode::OUT => 0b0001_0000,
            Opcode::DAT => 0b0001_0001,
        }
    }

    /// Generate from string
    pub fn from_str(string: &str) -> Opcode {
        match string {
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "STA" => Opcode::STA,
            "LDA" => Opcode::LDA,
            "JMP" => Opcode::JMP,
            "JEQ" => Opcode::JEQ,
            "JNE" => Opcode::JNE,
            "JGT" => Opcode::JGT,
            "JLT" => Opcode::JLT,
            "JZ" => Opcode::JZ,
            "JNZ" => Opcode::JNZ,
            "HLT" => Opcode::HLT,
            "INP" => Opcode::INP,
            "OUT" => Opcode::OUT,
            "DAT" => Opcode::DAT,
            _ => panic!("Invalid opcode"),
        }
    }
}

// FromStr for Opcode
impl std::str::FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Opcode::from_str(s))
    }
}