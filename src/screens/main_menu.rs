use macroquad::{
    math::vec2,
    ui::{Skin, Ui, widgets},
};

use crate::screens::Screen;

pub struct MainMenuScreen {
    skin: Skin,
}

impl MainMenuScreen {
    pub fn new(skin: Skin) -> Self {
        MainMenuScreen { skin }
    }

    pub async fn handle_event(&mut self) {}

    pub async fn draw(&mut self, ui: &mut Ui) -> Option<Screen> {
        let mut next_screen = None;
        ui.push_skin(&self.skin);

        if widgets::Button::new("Play")
            .position(vec2(0., 10.))
            .size(vec2(100., 30.))
            .ui(ui)
        {
            next_screen = Some(Screen::game(self.skin.clone()));
        }

        if widgets::Button::new("Options")
            .position(vec2(0., 50.))
            .size(vec2(100., 30.))
            .ui(ui)
        {
            println!("Options clicked (not implemented yet)");
        }

        if widgets::Button::new("Quit")
            .position(vec2(0., 90.))
            .size(vec2(100., 30.))
            .ui(ui)
        {
            std::process::exit(0);
        }

        ui.pop_skin();

        next_screen
    }
}
