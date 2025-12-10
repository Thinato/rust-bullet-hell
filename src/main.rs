use macroquad::prelude::*;

mod game;

fn window_conf() -> Conf {
    Conf {
        window_width: 800,
        window_height: 600,
        window_title: "Rust Bullet Hell".to_string(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    game::init_game();
    
    // loop {
    //     clear_background(BLACK);


    //     next_frame().await
    // }
}
