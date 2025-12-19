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
    bullet_spawn_deduction: f32,
    bullet_spawn_deduction_timer: f32,
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
            bullet_spawn_deduction: 0.2,
            bullet_spawn_deduction_timer: 5.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        if self.player.dead {
            return ScreenCommand::None;
        }

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
        self.player.update(dt);

        self.bullet_spawn_timer -= dt;
        self.bullet_spawn_deduction_timer -= dt;
        if self.bullet_spawn_timer <= 0.0 {
            self.bullets.push(Bullet::new_slow());
            self.bullet_spawn_timer = self.bullet_spawn_deduction;
        }

        if self.bullet_spawn_deduction_timer <= 0.0 {
            self.bullet_spawn_deduction -= 0.01;
            self.bullet_spawn_deduction_timer = 5.0;
            println!(
                "Bullet spawn deduction increased to {}",
                self.bullet_spawn_deduction
            );
        }

        self.bullets.retain(|bullet| !bullet.dead);

        for bullet in &mut self.bullets {
            bullet.update(dt);
            if self.player.rect().intersect(bullet.rect()).is_some() {
                self.player.take_damage(35);
            }
        }

        ScreenCommand::None
    }

    pub fn draw(&mut self, ui: &mut Ui) -> ScreenCommand {
        let mut command = ScreenCommand::None;
        let screen = vec2(screen_width(), screen_height());
        self.player.draw();
        self.bullets.iter_mut().for_each(|bullet| bullet.draw());
        ui.push_skin(&self.skin);

        draw_rectangle(0., 0., screen.x, 50., Color::from_rgba(45, 45, 45, 255));
        draw_rectangle(10., 10., 100., 30., Color::from_rgba(35, 35, 35, 255));
        draw_rectangle(
            10.,
            10.,
            self.player.health as f32 / self.player.max_health as f32 * 100.,
            30.,
            Color::from_rgba(0, 200, 0, 255),
        );
        let health_text = &format!("{}/{}", self.player.health, self.player.max_health);

        draw_text(
            health_text,
            60. - ((health_text.len() as f32 * 10.5) / 2.0),
            30.,
            24.,
            Color::from_rgba(255, 255, 255, 255),
        );

        if self.player.dead {
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

        if self.show_menu {
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
