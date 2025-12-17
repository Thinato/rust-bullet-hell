use crate::{
    entities::player::Player,
    screens::{Screen, ScreenCommand},
};
use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui, widgets};

pub struct GameScreen {
    skin: Skin,
    player: Player,
    show_menu: bool,
}

impl GameScreen {
    pub fn new(skin: Skin) -> Self {
        GameScreen {
            skin,
            player: Player::new(),
            show_menu: false,
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        if is_key_pressed(KeyCode::Escape) {
            self.show_menu = !self.show_menu;
        }
        
        if self.show_menu {
            return ScreenCommand::None;
        }

        let mut direction = Vec2::ZERO;


        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            direction.x += 1.0;
        }

        self.player.set_direction(direction);
        self.player.go(dt);

        ScreenCommand::None
    }

    pub fn draw(&mut self, ui: &mut Ui) -> ScreenCommand {
        self.player.draw();


        if self.show_menu {
            ui.push_skin(&self.skin);
            if widgets::Button::new("Back")
                .position(vec2(10., 10.))
                .size(vec2(100., 30.))
                .ui(ui)
            {
                ui.pop_skin();
                return ScreenCommand::Replace(Screen::main_menu(self.skin.clone()));
            }

            if widgets::Button::new("Quit")
                .position(vec2(10., 50.))
                .size(vec2(100., 30.))
                .ui(ui)
            {
                ui.pop_skin();
                return ScreenCommand::Quit;
            }
            ui.pop_skin();
        }

        ScreenCommand::None
    }
}
