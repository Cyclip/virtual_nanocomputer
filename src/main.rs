pub mod cpu;
pub mod assembler;

use cpu::instructions::{Instruction, Opcode};
use crate::cpu::registers::Register;

fn main() {
    let mut cpu = cpu::CPU::new(256, 256);

    // Create a new simple addition program
    let program = vec![
        Instruction::new(Opcode::LDA, 0x01),
        Instruction::new(Opcode::ADD, 0x02),
        Instruction::new(Opcode::OUT, 0x00),
        Instruction::new(Opcode::HLT, 0x00),
    ];

    // Assemble the program
    let binary = assembler::assemble(program);

    println!("Binary: {:?}", binary);

    // Load the program into the CPU
    cpu.load_program(binary);

    // Start the CPU
    cpu.start();

    // Print the result
    println!("Result: {}", cpu.acc.get());
    
    // Print memory
    println!("Data memory:\n{}", cpu.data_memory);
    println!("Instruction memory:\n{}", cpu.instruction_memory);
}
