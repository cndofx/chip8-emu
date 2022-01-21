use crate::memory::Memory;

use crate::{WIDTH, HEIGHT, PIXEL_COUNT};

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
        let mut x = x as usize % WIDTH;
        let y = y as usize % HEIGHT;

        for _ in 0..8 {
            let i = Display::index_from_coords(x, y);
            let old = self.screen[i];
            self.screen[i] = self.screen[i] ^ ((byte & 0b1000_0000) >> 7);
            overwritten = overwritten || (old == 1 && self.screen[i] == 0);
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

    pub fn clear(&mut self) {
        for i in 0..PIXEL_COUNT {
            self.screen[i] = 0;
        }
    }

    pub fn get_screen(&self) -> &[u8] {
        &self.screen
    }
}