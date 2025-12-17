pub mod director;
pub mod game;
pub mod main_menu;

use crate::{
    screens::{game::GameScreen, main_menu::MainMenuScreen},
};
use macroquad::ui::Skin;

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

    pub async fn draw_with_ui(&mut self, ui: &mut macroquad::ui::Ui) -> Option<Screen> {
        match self {
            Screen::MainMenu(screen) => screen.draw(ui).await,
            Screen::Game(screen) => screen.draw(ui).await,
        }
    }

    pub async fn handle_event(&mut self, dt: f32)  {
        match self {
            Screen::MainMenu(screen) => screen.handle_event().await,
            Screen::Game(screen) => screen.handle_event(dt).await,
        }
    }
}
