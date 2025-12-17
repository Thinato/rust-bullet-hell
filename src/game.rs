use crate::screens::director::Director;
use macroquad::prelude::*;

pub struct Game {
    director: Director,
}

impl Game {
    pub async fn new() -> Self {
        Game {
            director: Director::new().await,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let dt = get_frame_time();
            
            self.handle_event(dt).await.unwrap();

            clear_background(BLACK);
            self.director.draw().await;
            next_frame().await
        }
    }

    pub async fn handle_event(&mut self, dt: f32) -> Result<(), String> {
        // handle global events
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0)
        }

        self.director.handle_event(dt).await;
        Ok(())
    }
}
