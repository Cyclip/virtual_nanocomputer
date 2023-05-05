pub mod cpu;
pub mod assembler;

use cpu::instructions::{Instruction, Opcode};
use crate::cpu::registers::Register;

fn main() {
    // First assemble source code
    let binary = assembler::assemble("test.vnc");

    // Save binary to file
    assembler::save_to_file(binary, "test.bin");

    // Initialise CPU and load program
    let mut cpu = cpu::CPU::new(256, 256);
    cpu.load_program_from_file("test.bin");

    // Start CPU
    cpu.start();

    // Print acc
    println!("acc: {}", cpu.acc.get());

    // Print memory
    println!("memory: {}", cpu.data_memory);
}
