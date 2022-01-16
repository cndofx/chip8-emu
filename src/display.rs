use crate::memory::Memory;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const PIXEL_COUNT: usize = WIDTH * HEIGHT;

pub struct Display {
    screen: [u8; PIXEL_COUNT],
}

impl Display {
    pub fn new() -> Display {
        Display { screen: [0; PIXEL_COUNT] }
    }

    pub fn index_from_coords(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut overwritten = false;
        let mut byte = byte;
        let mut x = x as usize;
        let y = y as usize;

        for _ in 0..8 {
            let i = Display::index_from_coords(x, y);
            match (byte & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[i] == 1 {
                        overwritten = true;
                    }
                    self.screen[i] = 0;
                }
                1 => self.screen[i] = 1,
                _ => unreachable!(),
            }
            x += 1;
            byte = byte << 1;
        }

        overwritten
    }

    pub fn draw_sprite(&mut self, memory: &Memory, i: u16, x: u8, y: u8, height: u8) -> bool {
        let mut overwritten = false;
        for a in 0..height {
            let byte = memory.read_byte(i + a as u16);
            if self.draw_byte(byte, x, y + a) {
                overwritten = true;
            }
        }
        overwritten
    }

    pub fn debug_draw_screen(&mut self) {
        println!("   1   5    10   15   20   25   30   35   40   45   50   55   60  64");
        for y in 0..HEIGHT {
            print!("{:<3}", y + 1);
            for x in 0..WIDTH {
                let index = Display::index_from_coords(x as usize, y as usize);
                if self.screen[index] == 0 {
                    print!(".");
                }
                else {
                    print!("#");
                }
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn clear(&mut self) {
        for i in 0..PIXEL_COUNT {
            self.screen[i] = 0;
        }
    }
}