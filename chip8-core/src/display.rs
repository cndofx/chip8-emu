const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Debug)]
pub(crate) struct Display {
    pixels: Box<[u8]>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: vec![0; WIDTH * HEIGHT].into_boxed_slice(),
        }
    }

    pub fn get(&self) -> &[u8] {
        &self.pixels
    }

    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = 0;
        }
    }

    /// returns true if there was a collision
    pub fn draw(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        let mut erased = false;
        for (i, b) in sprite.iter().enumerate() {
            erased |= self.draw_byte(x, y + i as u8, *b);
        }
        erased
    }

    fn draw_byte(&mut self, x: u8, y: u8, mut byte: u8) -> bool {
        let mut x = x as usize % WIDTH;
        let y = y as usize % HEIGHT;
        let mut erased = false;
        for _ in 0..8 {
            let i = y * WIDTH + x;
            let old = self.pixels[i];
            self.pixels[i] ^= (byte & 0b10000000) >> 7;
            erased |= old == 1 && self.pixels[i] == 0;
            x += 1;
            byte <<= 1;
        }
        erased
    }

    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.pixels[y * WIDTH + x] == 0 {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}
