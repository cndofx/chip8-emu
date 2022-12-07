use byteorder::{ReadBytesExt, BigEndian};

use crate::{memory::Memory, bus::Bus};

#[derive(Debug)]
pub(crate) struct Cpu {
    pub bus: Bus,
    /// Program counter
    pc: u16,
    /// CPU registers
    vx: [u8; 16],
    /// I register
    i: u16,
    /// Sound timer
    st: u8,
    /// Delay timer
    dt: u8,
    // Stack pointer
    // sp: u8
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            bus: Bus::new(),
            vx: [0; 16],
            i: 0,
            st: 0,
            dt: 0,
            pc: 0x200,
            stack: Vec::with_capacity(16),
        }
    }

    pub fn fetch(&mut self) -> u16 {
        let mut instruction = self.bus.memory.read_slice(self.pc as usize, 2);
        instruction.read_u16::<BigEndian>().unwrap()
    }

    pub fn execute(&mut self, instruction: u16) {
        let nnn = instruction & 0x0FFF; // 12-bit address, lower 12 bits of instruction
        let kk = (instruction & 0x00FF) as u8; // 8-bit value, lower 8 bits of instruction
        let n = (instruction & 0x000F) as u8; // 4-bit value, lowest 4 bits of instruction
        let x = ((instruction & 0x0F00) >> 8) as u8; // 4-bit value, lower 4 bits of upper byte
        let y = ((instruction & 0x00F0) >> 4) as u8; // 4-bit value, upper 4 bits of lower byte

        println!("pc: 0x{:04X}", self.pc);
        println!("instruction: 0x{instruction:04X}");

        match (instruction & 0xF000) >> 12 {
            0x00 => match kk {
                0xE0 => {
                    println!("CLS");
                    self.bus.display.clear();
                    self.pc += 2;
                }
                _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
            }
            0x01 => {
                println!("JP {nnn:04X}");
                self.pc = nnn;
            }
            0x06 => {
                println!("LD V{x:X}, 0x{kk:02X}");
                self.vx[x as usize] = kk;
                self.pc += 2;
            }
            0x07 => {
                println!("ADD V{x:X}, 0x{kk:02X}");
                self.vx[x as usize] += kk; 
                self.pc += 2;
            }
            0x0A => {
                println!("LD I, 0x{nnn:04X}");
                self.i = nnn;
                self.pc += 2;
            }
            0x0D => {
                println!("DRW V{x:X}, V{y:X}, {n}");
                let vx = self.vx[x as usize];
                let vy = self.vx[y as usize];
                let sprite = self.bus.memory.read_slice(self.i as usize, n as usize);
                if self.bus.display.draw(vx, vy, sprite) {
                    self.vx[0xF] = 1;
                } else {
                    self.vx[0xF] = 0;
                }
                self.bus.display.print();
                self.pc += 2;
            }
            _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
        }

        println!("");

        self.tick()
        // self.pc += 2;
    }

    fn tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}
