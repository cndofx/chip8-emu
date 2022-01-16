use rand::Rng;

use crate::display::Display;
use crate::memory::Memory;

const ENTRY_POINT: u16 = 0x200;

pub struct CPU {
    display: Display,
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
            display: Display::new(),
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

        let nnn = instruction & 0x0FFF;              // 12-bit address, lowest 12 bits of instruction
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
            0x00 => {
                match kk {
                    0xE0 => {
                        // 00E0 --- CLS --- Clear the display
                        println!("Clearing the screen.");
                        self.display.clear();
                        self.pc += 2;
                    }
                    0xEE => {
                        // 00EE -- RET --- Return from a subroutine
                        let ret = self.stack.pop().expect("No value on the stack to pop.");
                        println!("Popped {:#X} from the stack.", ret);
                        println!("Returning from {:#X} to {:#X}.", self.pc, ret);
                        self.pc = ret;
                    }
                    _ => panic!("Unrecognized instruction {:#X} at {:#X}", instruction, self.pc),
                }
            }
            0x01 => {
                // 1nnn --- JP addr --- Jump to location nnn
                println!("Jumping to {:#X}.", nnn);
                self.pc = nnn;
            }
            0x02 => {
                // 2nnn --- CALL addr --- Call subroutine at nnn
                println!("Calling subroutine at {:#X}.", nnn);
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            }
            0x03 => {
                // 3xkk --- SE Vx, byte --- Skip next instruction if Vx = kk
                let vx = self.read_vx(x);
                println!("Testing if V{:X} ({}) == {:#X}", x, vx, kk);
                if vx == kk {
                    println!("Skipping next instruction.");
                    self.pc += 4;
                } else {
                    println!("Doing nothing.");
                    self.pc += 2;
                }
            }
            0x04 => {
                // 4xkk - SNE Vx, byte --- Skip next instruction if Vx != kk
                let vx = self.read_vx(x);
                println!("Testing if V{:X} ({}) != {:#X}", x, vx, kk);
                if vx != kk {
                    println!("Skipping next instruction.");
                    self.pc += 4;
                } else {
                    println!("Doing nothing.");
                    self.pc += 2;
                }
            }
            0x05 => {
                // 5xy0 --- SE Vx, Vy ---Skip next instruction if Vx = Vy
                println!("Testing if V{:X} == V{:X}", x, y);
                let vx = self.read_vx(x);
                let vy = self.read_vx(y);
                if vx == vy {
                    println!("Skipping next instruction.");
                    self.pc += 4;
                } else {
                    println!("Doing nothing.");
                    self.pc += 2;
                }
            }
            0x06 => {
                // 6xkk --- LD Vx, byte --- Set Vx = kk
                println!("Writing {:#X} to V{:X}.", kk, x);
                self.write_vx(x, kk);
                self.pc += 2;
            }
            0x07 => {
                // 7xkk --- ADD Vx, byte --- Set Vx = Vx + kk
                println!("Adding {:#X} to V{:X}", kk, x);
                let vx = self.read_vx(x);
                self.write_vx(x, vx.wrapping_add(kk));
                self.pc += 2;
            }
            0x08 => {
                match n {
                    0x00 => {
                        // 8xy0 --- LD Vx, Vy --- Set Vx = Vy
                        println!("Writing V{:X} to V{:X}", y, x);
                        let vy = self.read_vx(y);
                        self.write_vx(x, vy);
                    }
                    0x01 => {
                        // 8xy1 --- OR Vx, Vy --- Set Vx = Vx OR Vy
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        let val = vx | vy;
                        self.write_vx(x, val);
                        println!("Writing V{:X} OR V{:X} = {:#X} to V{:X}", x, y, val, x);
                    }
                    0x02 => {
                        // 8xy2 --- AND Vx, Vy -- Set Vx = Vx AND Vy
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        let val = vx & vy;
                        self.write_vx(x, val);
                        println!("Writing V{:X} AND V{:X} = {:#X} to V{:X}", x, y, val, x);
                    }
                    0x03 => {
                        // 8xy3 --- XOR Vx, Vy --- Set Vx = Vx XOR Vy
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        let val = vx ^ vy;
                        self.write_vx(x, val);
                        println!("Writing V{:X} XOR V{:X} = {:#X} to V{:X}", x, y, val, x);
                    }
                    0x04 => {
                        // 8xy4 --- ADD Vx, Vy --- Set Vx = Vx + Vy, set VF = carry
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        let val = vx as u16 + vy as u16;
                        if val > 255 {
                            println!("Setting VF flag to 1");
                            self.write_vx(0xF, 1);
                        }
                        else {
                            println!("Setting VF flag to 0");
                            self.write_vx(0xF, 0);
                        }
                        self.write_vx(x, val as u8);
                        println!("Writing V{:X} + V{:X} = {:#X}  to V{:X}", x, y, val, x);
                    }
                    0x05 => {
                        // 8xy5 --- Sub Vx, Vy --- Set Vx = Vx - Vy, set VF = NOT borrow
                        println!("Writing V{:X} - V{:X} to V{:X}", x, y, x);
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        if vx > vy {
                            println!("Setting VF flag to 1");
                            self.write_vx(0xF, 1);
                            self.write_vx(x, vx - vy);
                        }
                        else {
                            println!("Setting VF flag to 0");
                            self.write_vx(0xF, 0);
                            self.write_vx(x, 0);
                        }
                    }
                    0x06 => {
                        // 8xy6 --- SHR Vx {, Vy} --- Set Vx = Vx SHR 1
                        println!("Writing V{:X} SHR 1 to V{:X}", x, x);
                        let vx = self.read_vx(x);
                        let val = vx >> 1;
                        if vx & 0b1 == 1 {
                            println!("Setting VF to 1");
                            self.write_vx(0xF, 1);
                        }
                        else {
                            println!("Setting VF to 0");
                            self.write_vx(0xF, 0);
                        }
                        self.write_vx(x, val);
                    }
                    0x07 => {
                        // 8xy7 --- SUBN Vx, Vy --- Set Vx = Vy - Vx, set VF = NOT borrow
                        println!("Writing V{:X} - V{:X} to V{:X}", y, x, x);
                        let vx = self.read_vx(x);
                        let vy = self.read_vx(y);
                        if vy > vx {
                            println!("Setting VF flag to 1");
                            self.write_vx(0xF, 1);
                            self.write_vx(x, vy - vx);
                        }
                        else {
                            println!("Setting VF flag to 0");
                            self.write_vx(0xF, 0);
                            self.write_vx(x, 0);
                        }
                    }
                    0x0E => {
                        // 8xyE --- SHL Vx {, Vy} --- Set Vx = Vx SHL 1
                        println!("Writing V{:X} SHL 1 to V{:X}", x, x);
                        let vx = self.read_vx(x);
                        let val = vx << 1;
                        if (vx & 0b1000_0000) >> 7 == 1 {
                            println!("Setting VF to 1");
                            self.write_vx(0xF, 1);
                        }
                        else {
                            println!("Setting VF to 0");
                            self.write_vx(0xF, 0);
                        }
                        self.write_vx(x, val);
                    }
                    _ => panic!("Unrecognized instruction {:#X} at {:#X}", instruction, self.pc),
                }
                self.pc += 2;
            }
            0x09 => {
                // 9xy0 --- SNE Vx, Vy --- Skip next instruction if Vx != Vy
                println!("Testing if V{:X} != V{:X}", x, y);
                let vx = self.read_vx(x);
                let vy = self.read_vx(y);
                if vx != vy {
                    println!("Skipping next instruction");
                    self.pc += 4;
                } else {
                    println!("Doing nothing");
                    self.pc += 2;
                }
            }
            0x0A => {
                // Annn --- LD I, addr --- Set I = nnn
                println!("Writing {:#X} to i register", nnn);
                self.i = nnn;
                self.pc += 2;
            }
            0x0B => {
                // Bnnn --- JP V0, addr --- Jump to location nnn + V0
                println!("Jumping to {:#X} + V0", nnn);
                let v0 = self.read_vx(0);
                self.pc = nnn + v0 as u16;
            }
            0x0C => {
                // Cxkk --- RND Vx, byte --- Set Vx = random byte AND kk
                let random: u8 = rand::thread_rng().gen_range(0..=255);
                let val = random & kk;
                self.write_vx(x, val);
                self.pc += 2;
            }
            0x0D => {
                // Dxyn --- DRW Vx, Vy, n --- Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                println!("Drawing {}-byte sprite from i = {:#X}", n, self.i);
                let vx = self.read_vx(x);
                let vy = self.read_vx(y);
                self.display.draw_sprite(&self.memory, self.i, vx, vy, n);
                self.display.debug_draw_screen();
                self.pc += 2;
            }
            0x0E => {
                match kk {
                    0x9E => {
                        // Ex9E --- SKP Vx --- Skips the next instruction if the key with the value of Vx is pressed
                        println!("Skipping next instruction if the key in V{:X} is pressed", x);
                        unimplemented!();
                    }
                    0xA1 => {
                        // ExA1 --- SKNP Vx --- Skips the next instruction if the key with the value of Vx is not pressed
                        println!("Skipping next instruction if the key in V{:X} is not pressed", x);
                        unimplemented!();
                    }
                    _ => panic!("Unrecognized instruction {:#X} at {:#X}", instruction, self.pc),
                }
                self.pc += 2;
            }
            0x0F => {
                match kk {
                    0x07 => {
                        // Fx07 --- LD Vx, DT --- Set Vx = delay timer
                        println!("Setting V{:X} to delay timer", x);
                        self.write_vx(x, self.dt);
                    }
                    0x15 => {
                        // Fx15 --- LD DT, Vx --- Set delay timer = Vx
                        println!("Setting delay timer to V{:X}", x);
                        let vx = self.read_vx(x);
                        self.dt = vx;
                    }
                    0x1E => {
                        // Fx1E --- ADD I, Vx --- Set I = I + Vx
                        let vx = self.read_vx(x);
                        self.i += vx as u16;
                        println!("Adding V{:X} = {:#X} to i", x, vx);
                    }
                    0x65 => {
                        // Fx65 --- LD Vx, [i] --- Load values starting at address i into V0 through Vx
                        println!("Loading values from address i into V0 through V{:X}", x);
                        for k in 0..=x {
                            let value = self.memory.read_byte(self.i + k as u16);
                            self.write_vx(k, value);
                        }
                    }
                    _ => panic!("Unrecognized instruction {:#X}", instruction),
                }
                self.pc += 2;
            }
            _ => panic!("Unrecognized instruction {:#X} at {:#X}", instruction, self.pc),
        }

        self.tick();

        self.print_vx();
        self.print_stack();
    }

    fn tick(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    fn read_vx(&self, index: u8) -> u8 {
        self.vx[index as usize]
    }

    fn write_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    fn print_vx(&mut self) {
        println!("General purpose register states:\nV0:  {:#X}   V1:  {:#X}   V2:  {:#X}   V3:  {:#X}\nV4:  {:#X}   V5:  {:#X}   V6:  {:#X}   V7:  {:#X}\nV8:  {:#X}   V9:  {:#X}   VA:  {:#X}   VB:  {:#X}\nVC:  {:#X}   VD:  {:#X}   VE:  {:#X}   VF:  {:#X}", self.vx[0], self.vx[1], self.vx[2], self.vx[3], self.vx[4], self.vx[5], self.vx[6], self.vx[7], self.vx[8], self.vx[9], self.vx[10], self.vx[11], self.vx[12], self.vx[13], self.vx[14], self.vx[15]);
        println!("i = {:#X}", self.i);
        println!("dt = {}", self.dt);
        println!("st = {}", self.st);
    }

    fn print_stack(&mut self) {
        println!("Stack: {:#X?}", self.stack);
    }
}
