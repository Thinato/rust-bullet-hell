use crate::{entities::player::Player, screens::Screen};
use macroquad::prelude::*;
use macroquad::ui::Skin;

pub struct GameScreen {
    skin: Skin,
    player: Player,
}

impl GameScreen {
    pub fn new(skin: Skin) -> Self {
        GameScreen {
            skin,
            player: Player::new(),
        }
    }

    pub async fn handle_event(&mut self, dt: f32) {
        self.player.set_direction(Vec2::ZERO);
        
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            self.player.add_force(Vec2::new(0.0, -1.0));
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            self.player.add_force(Vec2::new(0.0, 1.0), );
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.player.add_force(Vec2::new(-1.0, 0.0), );
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.player.add_force(Vec2::new(1.0, 0.0), );
        }
        self.player.go(dt);
    }

    pub async fn draw(&mut self, _ui: &mut macroquad::ui::Ui) -> Option<Screen> {
        // TODO: Implement game rendering and transitions.
        // draw PowerUps
        // draw Bullets
        // draw Player
        self.player.draw().await;
        // draw UI
        None
    }
}
