use std::{path::{Path, PathBuf}, io::Read};

use crate::cpu::Cpu;

#[derive(Debug)]
pub struct Chip8 {
    /// CPU speed in Hz
    speed: u32,
    cpu: Cpu,
    rom_name: Option<String>,
    loaded: bool,
}

impl Chip8 {
    /// Set emulation clock speed in Hz.
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> 
    where
        PathBuf: From<P>
    {
        if self.loaded {
            self.reset();
        } 
        let path = PathBuf::from(path);
        let mut file = std::fs::File::open(&path)?;
        let mut buf: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        self.cpu.bus.memory.write_slice(0x200, &buf);
        self.loaded = true;
        self.rom_name = Some(path.file_name().unwrap().to_string_lossy().to_string());
        Ok(())
    }
    
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.loaded = false;
        self.rom_name = None;
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let ins = self.cpu.fetch();
        self.cpu.execute(ins);
    }

    pub fn get_display(&self) -> &[u8] {
        self.cpu.bus.display.get()
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_rom_name(&self) -> Option<String> {
        self.rom_name.clone()
    }

}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            speed: 500,
            cpu: Cpu::new(),
            loaded: false,
            rom_name: None,
        }
    }
}
