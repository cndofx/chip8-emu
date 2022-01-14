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
            0x00 => {
                match kk {
                    0xE0 => {
                        // 00E0 --- CLS --- Clear the display
                        println!("Clearing the screen.");
                        unimplemented!();
                    }
                    0xEE => {
                        // 00EE -- RET --- Return from a subroutine
                        let ret = self.stack.pop().expect("No value on the stack to pop.");
                        println!("Popped {:#X} from the stack.", ret);
                        println!("Returning from {:#X} to {:#X}.", self.pc, ret);
                        self.pc = ret;
                    }
                    _ => panic!("Unrecognized instruction {:#X}", instruction),
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
            _ => panic!("Unrecognized instruction {:#X}", instruction),
        }
        self.print_vx();
        self.print_stack();
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
