use std::{fs::File, io::Read};
use std::io::Write;
use crate::cpu::instructions::{Instruction, Opcode};

/// Assemble a program into a binary
pub fn assemble(instr: Vec<Instruction>) -> Vec<u8> {
    let mut binary = Vec::new();

    for instruction in instr {
        // get binary and push
        let opcode_bin = instruction.opcode.to_bin();
        let operand_bin = instruction.operand;

        binary.push(opcode_bin);
        binary.push(operand_bin);

        println!("{:?} -> {} {}", instruction, opcode_bin, operand_bin);
    };

    binary
}

/// Save a binary to a file
pub fn save_to_file(binary: Vec<u8>, filename: &str) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&binary).unwrap();
}

/// Load a binary from a file
pub fn load_from_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut binary = Vec::new();
    
    file.read_to_end(&mut binary).unwrap();

    binary
}