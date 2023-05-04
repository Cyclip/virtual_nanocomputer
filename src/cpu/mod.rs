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

/// Represents the CPU
pub struct CPU {

}