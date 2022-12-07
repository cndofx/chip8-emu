use chip8_core::chip8::Chip8;
use clap::Parser;
mod cli;

fn main() {
    let cli = cli::Cli::parse();
    let mut chip8 = Chip8::default();
    chip8.load_rom(cli.rom_path).unwrap();
    chip8.run();
}
