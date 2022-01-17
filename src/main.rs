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

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    //let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window = {
        let size = PhysicalSize::new(WIDTH as f64, HEIGHT as f64);
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
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            test_draw(pixels.get_frame());
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

            window.request_redraw();
        }
    });

    // let mut system: System = System::new();

    // let rom_file = File::open("roms/INVADERS").expect("Failed to open rom.");
    // let mut rom_reader = BufReader::new(rom_file);
    // let mut rom: Vec<u8> = Vec::new();
    // rom_reader.read_to_end(&mut rom).expect("Failed to read rom data.");

    // system.load_rom(&rom);
    // // system.run();
    // loop {
    //     system.execute();
    // }
}

fn test_draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        if i % 2 == 0 {
            pixel.copy_from_slice(&[0x0, 0x0, 0x0, 0x0]);
        } else {
            pixel.copy_from_slice(&[0x00, 0xFF, 0xFF, 0x2F]);
        }
    }
}
