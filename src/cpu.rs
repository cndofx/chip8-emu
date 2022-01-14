use crate::memory::Memory;

const ENTRY_POINT: u16 = 0x200;

pub struct CPU {
    pub memory: Memory,
    vx: [u8; 16],
    stack: Vec<u16>,
    i: u16,
    pc: u16,
    dt: u8,
    st: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            memory: Memory::new(),
            vx: [0; 16],
            stack: Vec::new(),
            i: 0,
            pc: ENTRY_POINT,
            dt: 0,
            st: 0,
        }
    }

    pub fn execute(&mut self) {
        let higher = self.memory.read_byte(self.pc) as u16;
        let lower = self.memory.read_byte(self.pc + 1) as u16;
        let instruction = (higher << 8) | lower;

        println!("\n==============================\n\n{:#X}... Read instruction {:#X} from lower {:#X} and higher {:#X}", self.pc, instruction, lower, higher);

        let nnn = instruction & 0x0FFF;             // 12-bit address, lowest 12 bits of instruction
        let n = (instruction & 0x000F) as u8;        // 4-bit value, lowest 4 bits of instruction
        let x = ((instruction & 0x0F00) >> 8) as u8; // 4-bit value, lower 4 bits of higher byte
        let y = ((instruction & 0x00F0) >> 4) as u8; // 4-bit value, higher 4 bits of lower byte
        let kk = (instruction & 0x00FF) as u8;       // 8-bit value, lowest 8 bits of instruction

        println!(
            "Parsed instruction: nnn = {}, n = {}, x = {}, y = {}, kk = {}",
            nnn, kk, n, x, y
        );

        match (instruction & 0xF000) >> 12 {
            // match the highest of the 4 nibbles
            _ => panic!("Unrecognized instruction {:#X}", instruction),
        }
    }
}
