use chip8_core::chip8::Chip8;
use clap::Parser;
mod cli;

use macroquad::prelude::*;

struct State {
    game_scale: f32,
    game_centered: bool,
    game_foreground_color: [f32; 3],
    game_background_color: [f32; 3],
    window_color: [f32; 3],
}

#[macroquad::main("Chip8")]
async fn main() {
    // let cli = cli::Cli::parse();
    // let mut chip8 = Chip8::default();
    // chip8.load_rom(cli.rom_path).unwrap();
    // chip8.run();

    let width = 64.0;
    let height = 32.0;

    let mut egui_state = State {
        game_scale: 10.0,
        game_centered: true,
        window_color: [0.1; 3],
        game_foreground_color: [1.0; 3],
        game_background_color: [0.0; 3],
    };

    loop {
        let window_background_color = Color::new(egui_state.window_color[0], egui_state.window_color[1], egui_state.window_color[2], 1.0);
        let game_foreground_color = Color::new(egui_state.game_foreground_color[0], egui_state.game_foreground_color[1], egui_state.game_foreground_color[2], 1.0);
        let game_background_color = Color::new(egui_state.game_background_color[0], egui_state.game_background_color[1], egui_state.game_background_color[2], 1.0);

        clear_background(window_background_color
        );

        let mut image = Image::gen_image_color(64, 32, game_background_color);

        let texture = Texture2D::from_image(&image);

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

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Chip8 Emulator").show(ctx, |ui| {
                // ui.label("test");
                ui.collapsing("Settings", |ui| {
                    ui.add(egui::Slider::new(&mut egui_state.game_scale, 1.0..=50.0).text("Game Scale"));
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
            });
        });

        egui_macroquad::draw();

        next_frame().await
    }
}
