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