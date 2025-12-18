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
        let center = vec2(screen_width() * 0.5, screen_height() * 0.5);

        GameScreen {
            skin,
            player: Player::new_at(center),
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

            let screen = vec2(screen_width(), screen_height());
            let button_size = vec2(120., 30.);
            let spacing = 12.;
            let column_height = 2. * button_size.y + spacing;
            let start = vec2(
                (screen.x - button_size.x) * 0.5,
                (screen.y - column_height) * 0.5,
            );

            if widgets::Button::new("Back")
                .position(start)
                .size(button_size)
                .ui(ui)
            {
                ui.pop_skin();
                return ScreenCommand::Replace(Screen::main_menu(self.skin.clone()));
            }

            if widgets::Button::new("Quit")
                .position(vec2(start.x, start.y + button_size.y + spacing))
                .size(button_size)
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
