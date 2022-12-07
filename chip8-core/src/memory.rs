const MEMORY_SIZE: usize = 4096; // 4 KiB

#[derive(Debug)]
pub(crate) struct Memory {
    memory: Box<[u8]>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            memory: vec![0; MEMORY_SIZE].into_boxed_slice(),
        }
    }

    /// Load standard font into memory at 0x00..0x80
    pub fn load_font(&mut self) {
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        self.write_slice(0, &font);
    }

    /// Read byte from given address.
    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    /// Read slice from given address.
    pub fn read_slice(&self, address: usize, length: usize) -> &[u8] {
        &self.memory[address..address + length]
    }

    /// Write byte at given address.
    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    /// Write slice into memory at given address.
    pub fn write_slice(&mut self, address: usize, value: &[u8]) {
        let size = value.len();
        self.memory[address..address + size].copy_from_slice(value);
    }

    /// Set `length` bytes to `value` at `address`.
    pub fn set(&mut self, address: usize, length: usize, value: u8) {
        for i in 0..length {
            self.memory[address + i] = value;
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Memory::new();
        memory.load_font();
        memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_write() {
        let mut mem = Memory::new();

        mem.write_byte(0, 1);
        mem.write_slice(2, &[1, 2, 3, 4, 5]);
        mem.set(8, 4, 9);

        assert_eq!(mem.read_slice(0, 12), &[1, 0, 1, 2, 3, 4, 5, 0, 9, 9, 9, 9]);
    }
}
