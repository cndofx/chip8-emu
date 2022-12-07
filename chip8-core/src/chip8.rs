use std::{path::Path, io::Read};

use crate::memory::Memory;

#[derive(Debug)]
pub struct Chip8 {
    /// CPU speed in Hz
    speed: u32,
    memory: Memory,
}

impl Chip8 {
    /// Set emulation clock speed in Hz.
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        self.memory.write_slice(0x200, &buf);
        Ok(())
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            speed: 500,
            memory: Memory::default()
        }
    }
}
