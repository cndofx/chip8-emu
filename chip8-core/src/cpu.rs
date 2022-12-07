use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;

use crate::bus::Bus;

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

    pub fn reset(&mut self) {
        *self = Cpu::new();
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

        self.print_state();
        println!("Instruction: 0x{instruction:04X}");

        match (instruction & 0xF000) >> 12 {
            0x00 => match kk {
                0xE0 => {
                    println!("CLS");
                    self.bus.display.clear();
                    self.pc += 2;
                }
                0xEE => {
                    println!("RET");
                    self.pc = self.stack.pop().unwrap();
                }
                _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
            },
            0x01 => {
                println!("JP {nnn:04X}");
                self.pc = nnn;
            }
            0x02 => {
                println!("CALL {nnn:04X}");
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x03 => {
                println!("SE V{x:X}, {kk:02X}");
                if self.vx[x as usize] == kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x04 => {
                println!("SNE V{x:X}, {kk:02X}");
                if self.vx[x as usize] != kk {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x05 => {
                println!("SE V{x:X}, V{y:X}");
                if self.vx[x as usize] == self.vx[y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x06 => {
                println!("LD V{x:X}, 0x{kk:02X}");
                self.vx[x as usize] = kk;
                self.pc += 2;
            }
            0x07 => {
                println!("ADD V{x:X}, 0x{kk:02X}");
                self.vx[x as usize] = self.vx[x as usize].wrapping_add(kk);
                self.pc += 2;
            }
            0x08 => match instruction & 0x000F {
                0x00 => {
                    println!("LD V{x:X}, V{y:X}");
                    self.vx[x as usize] = self.vx[y as usize];
                    self.pc += 2;
                }
                0x01 => {
                    println!("OR V{x:X}, V{y:X}");
                    self.vx[x as usize] |= self.vx[y as usize];
                    self.pc += 2;
                }
                0x02 => {
                    println!("AND V{x:X}, V{y:X}");
                    self.vx[x as usize] &= self.vx[y as usize];
                    self.pc += 2;
                }
                0x03 => {
                    println!("XOR V{x:X}, V{y:X}");
                    self.vx[x as usize] ^= self.vx[y as usize];
                    self.pc += 2;
                }
                0x04 => {
                    println!("ADD V{x:X}, V{y:X}");
                    let sum = self.vx[x as usize] as u16 + self.vx[y as usize] as u16;
                    if sum > 255 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[x as usize] = sum as u8;
                    self.pc += 2;
                }
                0x05 => {
                    println!("SUB V{x:X}, V{y:X}");
                    println!("vx = {}, vy = {}", self.vx[x as usize], self.vx[y as usize]);
                    if self.vx[x as usize] > self.vx[y as usize] {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[x as usize] = self.vx[x as usize].wrapping_sub(self.vx[y as usize]);
                    self.pc += 2;
                }
                0x06 => {
                    println!("SHR V{x:X} {{, V{y:X}}}");
                    if self.vx[x as usize] & 0b00000001 != 0 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[x as usize] >>= 1;
                    self.pc += 2;
                }
                0x07 => {
                    println!("SUBN V{x:X}, V{y:X}");
                    if self.vx[y as usize] > self.vx[x as usize] {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[y as usize] = self.vx[y as usize].wrapping_sub(self.vx[x as usize]);
                    self.pc += 2;
                }
                0x0E => {
                    println!("SHL V{x:X} {{, V{y:X}}}");
                    if self.vx[x as usize] & 0b10000000 != 0 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[x as usize] <<= 1;
                    self.pc += 2;
                }
                _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
            },
            0x09 => {
                println!("SNE V{x:X}, V{y:X}");
                if self.vx[x as usize] != self.vx[y as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            0x0A => {
                println!("LD I, 0x{nnn:04X}");
                self.i = nnn;
                self.pc += 2;
            }
            0x0C => {
                println!("RND V{x:X}, {kk:02X}");
                self.vx[x as usize] = rand::thread_rng().gen_range(0..=255) & kk;
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
                // self.bus.display.print();
                self.pc += 2;
            }
            0x0E => match kk {
                0xA1 => {
                    println!("SKNP V{x:X}");
                    if !self.bus.keyboard.is_pressed(self.vx[x as usize]) {
                        self.pc += 2;
                    }
                    self.pc += 2;
                }
                _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
            }
            0x0F => match kk {
                0x07 => {
                    println!("LD V{x:X}, DT");
                    self.vx[x as usize] = self.dt;
                    self.pc += 2;
                }
                0x15 => {
                    println!("LD DT, V{x:X}");
                    self.dt = self.vx[x as usize];
                    self.pc += 2;
                }
                0x1E => {
                    println!("ADD I, V{x:X}");
                    self.i = self.i.wrapping_add(self.vx[x as usize] as u16);
                    self.pc += 2;
                }
                0x29 => {
                    println!("LD F, V{x:X}");
                    self.i = self.vx[x as usize] as u16 * 5;
                    self.pc += 2;
                }
                0x33 => {
                    println!("LD B, V{x:X}");
                    let vx = self.vx[x as usize];
                    let hundreds = (vx / 100) % 10;
                    let tens = (vx / 10) % 10;
                    let ones = (vx / 1) % 10;
                    self.bus.memory.write_slice(self.i as usize, &[hundreds, tens, ones]);
                    self.pc += 2;
                }
                0x55 => {
                    println!("LD [I], V{x:X}");
                    for i in 0..=x as usize {
                        self.bus.memory.write_byte(self.i as usize + i, self.vx[i]);
                    }
                    self.pc += 2;
                }
                0x65 => {
                    println!("LD V{x:X}, [I]");
                    for i in 0..=x as usize {
                        self.vx[i] = self.bus.memory.read_byte(self.i as usize + i);
                    }
                    self.pc += 2;
                }
                _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
            },
            _ => panic!("unrecognized instruction: 0x{instruction:04X?}"),
        }

        println!("");

        self.tick()
    }

    fn tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn print_state(&self) {
        println!("CPU State:");
        println!("VX: {:02X?}", self.vx);
        println!("Stack: {:02X?}", self.stack);
        println!("PC: 0x{:04X}, I: 0x{:04X}", self.pc, self.i);
        println!("DT: 0x{:02X}, ST: 0x{:02X}", self.dt, self.st);
    }
}
