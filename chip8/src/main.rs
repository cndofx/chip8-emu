use chip8_core::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::default();
    chip8.set_speed(1000);

    println!("{:?}", chip8);
}
