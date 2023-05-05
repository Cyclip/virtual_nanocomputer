pub mod cpu;
pub mod assembler;

use cpu::instructions::{Instruction, Opcode};
use crate::cpu::registers::Register;

fn main() {
    // First assemble source code
    let binary = assembler::assemble("test.vnc");

    // Save binary to file
    assembler::save_to_file(binary, "test.bin");
}
