use std::fs::File;
use std::io::{BufReader, Read};

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::VirtualKeyCode;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use system::System;
use winit_input_helper::WinitInputHelper;

mod cpu;
mod display;
mod memory;
mod system;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
pub const PIXEL_COUNT: usize = WIDTH * HEIGHT;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = PhysicalSize::new((WIDTH*20) as f64, (HEIGHT*20) as f64);
        WindowBuilder::new()
            .with_title("CHIP8 Emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut system: System = System::new();

    let rom_file = File::open("roms/INVADERS").expect("Failed to open rom.");
    let mut rom_reader = BufReader::new(rom_file);
    let mut rom: Vec<u8> = Vec::new();
    rom_reader.read_to_end(&mut rom).expect("Failed to read rom data.");

    system.load_rom(&rom);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            draw_screen(pixels.get_frame(), &system);
            pixels.render().unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }
            for _ in 0..4 {
                system.execute();
            }
            window.request_redraw();
        }
    });

}

fn draw_screen(frame: &mut [u8], system: &System) {
    let screen = system.get_screen();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if screen[i] == 0 {
            pixel.copy_from_slice(&[0x0, 0x0, 0x0, 0x0]);
        } else {
            pixel.copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
        }
    }
}