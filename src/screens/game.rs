use crate::{
    entities::{bullet::Bullet, player::Player},
    screens::{Screen, ScreenCommand},
};
use macroquad::prelude::*;
use macroquad::ui::{Skin, Ui, widgets};

pub struct GameScreen {
    skin: Skin,
    player: Player,
    bullets: Vec<Bullet>,
    bullet_spawn_timer: f32,
    show_menu: bool,
}

impl GameScreen {
    pub fn new(skin: Skin) -> Self {
        let center = vec2(screen_width() * 0.5, screen_height() * 0.5);
        let player = Player::new_at(center);
        let bullets = Vec::new();
        let bullet_spawn_timer = 0.1;

        GameScreen {
            skin,
            player,
            bullets,
            bullet_spawn_timer,
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

        self.bullet_spawn_timer -= dt;
        if self.bullet_spawn_timer <= 0.0 {
            self.bullets.push(Bullet::new_slow());
            self.bullet_spawn_timer = 0.2;
        }

        self.bullets.retain(|bullet| !bullet.dead);

        self.bullets.iter_mut().for_each(|bullet| bullet.update(dt));

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
        let mut command = ScreenCommand::None;
        self.player.draw();
        self.bullets.iter_mut().for_each(|bullet| bullet.draw());
        ui.push_skin(&self.skin);

        if self.show_menu {
            let screen = vec2(screen_width(), screen_height());
            let button_size = vec2(150., 42.);
            let spacing = 18.;
            let padding = 52.;
            let column_height = 2. * button_size.y + spacing;
            let window_size = vec2(button_size.x + padding * 2., column_height + padding * 2.);
            let window_pos = (screen - window_size) * 0.5;
            let inner_start = vec2(
                (window_size.x - button_size.x) * 0.5,
                (window_size.y - column_height) * 0.5,
            );

            widgets::Window::new(1, window_pos, window_size)
                .movable(false)
                .titlebar(false)
                .ui(ui, |ui| {
                    if widgets::Button::new("Back")
                        .position(inner_start)
                        .size(button_size)
                        .ui(ui)
                    {
                        command = ScreenCommand::Replace(Screen::main_menu(self.skin.clone()));
                    }

                    if widgets::Button::new("Quit")
                        .position(vec2(inner_start.x, inner_start.y + button_size.y + spacing))
                        .size(button_size)
                        .ui(ui)
                    {
                        command = ScreenCommand::Quit;
                    }
                });
        }
        ui.pop_skin();

        command
    }
}
