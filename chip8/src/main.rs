use chip8_core::chip8::Chip8;
use clap::Parser;
mod cli;

use egui_file::FileDialog;
use macroquad::prelude::*;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

struct State {
    target_fps: f32,
    cycles_per_frame: u32,
    paused: bool,
    game_scale: f32,
    game_centered: bool,
    game_foreground_color: [f32; 3],
    game_background_color: [f32; 3],
    window_color: [f32; 3],
    file_dialog: Option<FileDialog>,
}

#[macroquad::main("Chip8")]
async fn main() {
    let cli = cli::Cli::parse();
    let mut chip8 = Chip8::default();
    if let Some(path) = cli.rom_path {
        chip8.load_rom(path).unwrap();
    }

    let mut egui_state = State {
        target_fps: 60.0,
        cycles_per_frame: 1,
        paused: false,
        game_scale: 10.0,
        game_centered: true,
        window_color: [0.1; 3],
        game_foreground_color: [1.0; 3],
        game_background_color: [0.0; 3],
        file_dialog: None,
    };

    let width = WIDTH as f32;
    let height = HEIGHT as f32;
    let mut total_cycles = 0u64;
    let mut last_time = 0.0;
    // let mut done = false;

    loop {

        // if done {
        //     break;
        // }

        if !egui_state.paused && chip8.is_loaded() {
            for _ in 0..egui_state.cycles_per_frame {
                chip8.step();
                total_cycles = total_cycles.wrapping_add(1);
            }
        }

        let window_background_color = Color::new(
            egui_state.window_color[0],
            egui_state.window_color[1],
            egui_state.window_color[2],
            1.0,
        );
        let game_foreground_color = Color::new(
            egui_state.game_foreground_color[0],
            egui_state.game_foreground_color[1],
            egui_state.game_foreground_color[2],
            1.0,
        );
        let game_background_color = Color::new(
            egui_state.game_background_color[0],
            egui_state.game_background_color[1],
            egui_state.game_background_color[2],
            1.0,
        );

        clear_background(window_background_color);

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Chip8 Emulator").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.add(egui::Button::new("Load ROM")).clicked() {
                        let mut dialog = FileDialog::open_file(None);
                        dialog.open();
                        egui_state.file_dialog = Some(dialog);
                    }
                    if ui.add(egui::Button::new("Reset")).clicked() {
                        chip8.reset();
                        total_cycles = 0;
                    }
                    if let Some(dialog) = &mut egui_state.file_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                chip8.load_rom(file).unwrap();
                                total_cycles = 0;
                            }
                        }
                    }
                });
                ui.label(if chip8.is_loaded() {
                    format!("{} loaded", chip8.get_rom_name().unwrap())
                } else {
                    "no rom loaded".to_string()
                });
                ui.add(egui::Checkbox::new(&mut egui_state.paused, "Paused"));
                ui.collapsing("Settings", |ui| {
                    ui.add(
                        egui::Slider::new(&mut egui_state.target_fps, 1.0..=200.0)
                            .text("Target FPS"),
                    );
                    ui.add(
                        egui::Slider::new(&mut egui_state.cycles_per_frame, 1..=20)
                            .text("Cycles per frame"),
                    );
                    ui.add(
                        egui::Slider::new(&mut egui_state.game_scale, 1.0..=50.0)
                            .text("Game Scale"),
                    );
                    ui.add(egui::Checkbox::new(
                        &mut egui_state.game_centered,
                        "Game Centered",
                    ));
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut egui_state.game_foreground_color);
                        ui.label("Game Foreground Color");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut egui_state.game_background_color);
                        ui.label("Game Background Color");
                    });
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut egui_state.window_color);
                        ui.label("Window Background Color");
                    });
                });
                // if ui.add(egui::Button::new("Exit")).clicked() {
                //     done = true;
                // }
                ui.label(format!("{} fps, {} cycles", get_fps(), total_cycles));
            });
        });

        // render game to texture
        let mut image = Image::gen_image_color(64, 32, game_background_color);
        render(
            &mut image,
            chip8.get_display(),
            game_foreground_color,
            game_background_color,
        );
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);

        // render texture to screen
        draw_texture_ex(
            texture,
            if egui_state.game_centered {
                screen_width() / 2.0 - texture.width() * egui_state.game_scale / 2.0
            } else {
                0.0
            },
            if egui_state.game_centered {
                screen_height() / 2.0 - texture.height() * egui_state.game_scale / 2.0
            } else {
                0.0
            },
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: width * egui_state.game_scale,
                    y: height * egui_state.game_scale,
                }),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );

        // render egui menus to screen
        egui_macroquad::draw();

        // fps limit
        let target_time = 1.0 / egui_state.target_fps as f64;
        let delta_time = get_time() - last_time;
        if delta_time < target_time {
            let sleep_time = target_time - delta_time;
            std::thread::sleep(std::time::Duration::from_micros((sleep_time * 1000000.0) as u64));
        }
        last_time = get_time();


        next_frame().await
    }
}

fn render(image: &mut Image, pixels: &[u8], foreground: Color, background: Color) {
    for (i, p) in pixels.iter().enumerate() {
        if *p == 0 {
            image.set_pixel((i % WIDTH) as u32, (i / WIDTH) as u32, background);
        } else {
            image.set_pixel((i % WIDTH) as u32, (i / WIDTH) as u32, foreground);
        }
    }
}
