use std::{path::Path, io::Read};

use crate::{memory::Memory, cpu::Cpu, display::Display};

#[derive(Debug)]
pub struct Chip8 {
    /// CPU speed in Hz
    speed: u32,
    cpu: Cpu,
    // memory: Memory,
    // display: Display,
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
        self.cpu.bus.memory.write_slice(0x200, &buf);
        Ok(())
    }

    pub fn run(&mut self) {
        loop {
            let ins = self.cpu.fetch();
            self.cpu.execute(ins);
        }
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            speed: 500,
            cpu: Cpu::new(),
        }
    }
}
