use crate::memory::Memory;

#[derive(Debug)]
pub struct Chip8 {
    /// cpu speed in Hz
    speed: u32,
    memory: Memory,
}

impl Chip8 {
    /// Set emulation clock speed in Hz.
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut memory = Memory::new();
        memory.load_font();
        Self {
            speed: 500,
            memory,
        }
    }
}
