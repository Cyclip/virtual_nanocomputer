//! Represents structure of main memory
//! This is split into 2 (similar to Harvard Architecture)
//! 1. Data Memory (stores data, label pointers)
//! 2. Instruction Memory (stores instructions)

/// All memory instructions
pub struct Memory {
    /// Memory size
    pub size: u32,
    /// Memory data
    pub data: Vec<u8>,
}

impl Memory {
    /// Create new memory
    pub fn new(size: u32) -> Memory {
        Memory {
            size,
            data: vec![0; size as usize],
        }
    }

    /// Read a byte from memory
    pub fn read(&self, address: u32) -> u8 {
        self.data[address as usize]
    }

    /// Write a byte to memory
    pub fn write(&mut self, address: u32, value: u8) {
        self.data[address as usize] = value;
    }

    /// Read a word from memory
    /// (2 bytes)
    pub fn read_word(&self, address: u32) -> u16 {
        let byte1 = self.read(address) as u16;
        let byte2 = self.read(address + 1) as u16;
        (byte1 << 8) | byte2
    }

    /// Write a word to memory
    /// (2 bytes)
    pub fn write_word(&mut self, address: u32, value: u16) {
        let byte1 = (value >> 8) as u8;
        let byte2 = value as u8;
        self.write(address, byte1);
        self.write(address + 1, byte2);
    }
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Memory {{ size: {}, data: {:?} }}", self.size, self.data)
    }
}

// pretty print
// Example output:
// Address | Value | Address | Value
// ----------------------------------
// 0x0000  | 0x01  | 0x0010  | 0x00
// 0x0001  | 0x02  | 0x0011  | 0x00

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        let mut i = 0;
        while i < self.size {
            let address = format!("0x{:04X}", i);
            let value = format!("0x{:02X}", self.read(i));
            output.push_str(&format!("{:<8} | {:<8}", address, value));
            i += 1;
            if i % 2 == 0 {
                output.push_str("\n");
            } else {
                output.push_str(" | ");
            }
        }
        write!(f, "{}", output)
    }
}