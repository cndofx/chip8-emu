use std::fs::File;
use std::io::{BufReader, Read};

use system::System;

mod cpu;
mod memory;
mod system;

fn main() {
    let mut system: System = System::new();
    
    let rom_file = File::open("roms/INVADERS").expect("Failed to open rom.");
    let mut rom_reader = BufReader::new(rom_file);
    let mut rom: Vec<u8> = Vec::new();
    rom_reader.read_to_end(&mut rom).expect("Failed to read rom data.");

    system.load_rom(&rom);
    system.run();
}
