use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{cpu::Cpu, keyboard::Keyboard};

#[derive(Debug)]
pub struct Chip8 {
    /// CPU speed in Hz
    speed: u32,
    cpu: Cpu,
    rom_path: Option<PathBuf>,
    loaded: bool,
}

impl Chip8 {
    /// Set emulation clock speed in Hz.
    pub fn set_speed(&mut self, speed: u32) {
        self.speed = speed;
    }

    pub fn load_rom<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()>
    where
        PathBuf: From<P>,
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
        self.rom_path = Some(path);
        Ok(())
    }

    pub fn reload_rom(&mut self) -> std::io::Result<()> {
        if let Some(rom) = self.rom_path.clone() {
            self.load_rom(&rom)?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "can't reload rom if no rom is loaded",
            ))
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.loaded = false;
        self.rom_path = None;
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

    pub fn set_key_state(&mut self, key: usize, state: bool) {
        self.cpu.bus.keyboard.set_key_state(key, state);
    }

    pub fn get_display(&self) -> &[u8] {
        self.cpu.bus.display.get()
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn get_rom_name(&self) -> Option<String> {
        if let Some(path) = self.rom_path.clone() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            Some(name)
        } else {
            None
        }
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            speed: 500,
            cpu: Cpu::new(),
            loaded: false,
            rom_path: None,
        }
    }
}
