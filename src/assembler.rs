//! Assembles a source file into a binary file
//! 0x01 and onwards are all instructions
//! 0x00 denotes the split between data and code
//! Data comes first, then code
//! 
//! Example source file:
//! ```
//! .data
//!     A	DAT	4
//!     B	DAT	2
//! 
//! .code
//!     LDA A
//!     ADD B
//!     OUT
//!     HLT
//! ```
//! 
//! This will compile in the following format:
//! (binary_letter value)* (opcode operand)*
//! binary_letter: the variable name in ASCII binary
//! value: the value of the variable in binary
//! opcode: the opcode in binary
//! operand: the operand in binary


use std::{fs::File, io::Read};
use std::io::Write;
use crate::cpu::instructions::{Instruction, Opcode};

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

/// Current section of source file
#[derive(PartialEq)]
enum CurrentSection {
    Data,
    Code,
    None,
}

#[derive(Debug)]
pub struct CodeLine {
    pub label: Option<String>,
    pub opcode: Option<Opcode>,
    pub operand: Option<OperandType>,
}

#[derive(Debug)]
pub enum OperandType {
    Label(String),
    Value(u8),
}

#[derive(Debug)]
pub struct DataLine {
    pub label: Option<String>,
    pub value: Option<u8>,
}

/// Assemble a source file into a binary file
pub fn assemble(source_path: &str) -> Vec<u8> {
    // Read source file
    let source = clean_source(
        &read_file(source_path)
    );

    // Split source file into lines
    let lines = source.split("\n");

    // Turn into sections
    let mut data_section: Vec<DataLine> = Vec::new();
    let mut code_section: Vec<CodeLine> = Vec::new();

    let mut current_section = CurrentSection::None;

    for line in lines {
        // ignore if empty or comment
        if line == "" || line.starts_with("//") {
            continue;
        }

        let line = clean_source(line);

        // Check if section
        if line.starts_with(".") {
            // Check if data section
            if line.starts_with(".data") {
                current_section = CurrentSection::Data;
                continue;
            }

            // Check if code section
            if line.starts_with(".code") {
                current_section = CurrentSection::Code;
                continue;
            }
        } else if line.starts_with(" ") {
            // is under a section
            let mut parts = line.split(" ");
            // skip first
            parts.next();

            match current_section {
                CurrentSection::Data => {
                    // is data
                    // LABEL DAT VALUE
                    let label: Option<String> = parts.next().map(|s| s.to_string());
                    parts.next(); // DAT
                    let operand: Option<u8> = parts.next().map(|s| s.parse::<u8>().unwrap());

                    data_section.push(DataLine {
                        label: label,
                        value: operand,
                    });
                },
                CurrentSection::Code => {
                    // is code
                    // LABEL OPCODE OPERAND
                    let label: Option<String> = parts.next().map(|s| s.to_string());
                    let opcode: Option<Opcode> = parts.next().map(|s| s.parse::<Opcode>().unwrap());
                    let operand: Option<OperandType> = parts.next().map(|s| {
                        if s.starts_with("0x") {
                            OperandType::Value(s.replace("0x", "").parse::<u8>().unwrap())
                        } else {
                            OperandType::Label(s.to_string())
                        }
                    });

                    code_section.push(CodeLine {
                        label: label,
                        opcode: opcode,
                        operand: operand,
                    });
                },
                CurrentSection::None => {
                    // is invalid
                    panic!("Invalid section");
                },
            }
        }
    }

    // Assemble data section
    let mut binary: Vec<u8> = Vec::new();

    // Add data section
    // Format: (binary_letter value)*
    for line in data_section {
        // Add label
        if let Some(label) = line.label {
            // Add label
            for c in label.chars() {
                binary.push(c as u8);
            }
        }

        // Add value
        if let Some(value) = line.value {
            binary.push(value);
        }
    }

    // Add code section
    // Format: (opcode operand)*
    for line in code_section {
        // Add opcode
        if let Some(opcode) = line.opcode {
            binary.push(opcode as u8);
        }

        // Add operand
        if let Some(operand) = line.operand {
            match operand {
                OperandType::Label(label) => {
                    // Add label
                    for c in label.chars() {
                        binary.push(c as u8);
                    }
                },
                OperandType::Value(value) => {
                    // Add value
                    binary.push(value);
                },
            }
        }
    }

    binary
}

/// Read a file
fn read_file(source: &str) -> String {
    let mut file = File::open(source).unwrap();
    let mut source = String::new();
    file.read_to_string(&mut source).unwrap();

    source
}

/// Clean a source file
/// Removes comments and whitespace
fn clean_source(source: &str) -> String {
    let mut source = source.to_string();

    // Remove comments
    source = source.replace("//", "\n");

    // Remove all duplicate whitespace
    source = source.replace("  ", " ");

    // Remove /r
    source = source.replace("\r", "");

    source
}