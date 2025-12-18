use macroquad::prelude::*;

mod entities;
mod game;
mod screens;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        window_title: "Rust Bullet Hell".to_string(),
        platform: miniquad::conf::Platform {
            // Prefer Metal; patched miniquad will fall back to OpenGL if Metal is unavailable.
            apple_gfx_api: miniquad::conf::AppleGfxApi::Metal,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    game::Game::new().await.start().await;
}
