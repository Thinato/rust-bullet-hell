pub mod director;
pub mod game;
pub mod main_menu;

use crate::{
    screens::{game::GameScreen, main_menu::MainMenuScreen},
};
use macroquad::ui::Skin;

pub enum ScreenCommand {
    None,
    Replace(Screen),
    Quit,
}

pub enum Screen {
    MainMenu(MainMenuScreen),
    Game(GameScreen),
    // GameOver(GameOverScreen),
    // HighScores(HighScoresScreen),
    // Settings(SettingsScreen),
    // Credits(CreditsScreen),
}

impl Screen {
    pub fn main_menu(skin: Skin) -> Self {
        Screen::MainMenu(MainMenuScreen::new(skin))
    }

    pub fn game(skin: Skin) -> Self {
        Screen::Game(GameScreen::new(skin))
    }

    pub fn draw_with_ui(&mut self, ui: &mut macroquad::ui::Ui) -> ScreenCommand {
        match self {
            Screen::MainMenu(screen) => screen.draw(ui),
            Screen::Game(screen) => screen.draw(ui),
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        match self {
            Screen::MainMenu(screen) => screen.update(dt),
            Screen::Game(screen) => screen.update(dt),
        }
    }
}
