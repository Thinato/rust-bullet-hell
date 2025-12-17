use macroquad::{
    math::vec2,
    ui::{Skin, Ui, widgets},
};

use crate::screens::{Screen, ScreenCommand};

pub struct MainMenuScreen {
    skin: Skin,
}

impl MainMenuScreen {
    pub fn new(skin: Skin) -> Self {
        MainMenuScreen { skin }
    }

    pub fn update(&mut self, _dt: f32) -> ScreenCommand {
        ScreenCommand::None
    }

    pub fn draw(&mut self, ui: &mut Ui) -> ScreenCommand {
        ui.push_skin(&self.skin);

        if widgets::Button::new("Play")
            .position(vec2(0., 10.))
            .size(vec2(100., 30.))
            .ui(ui)
        {
            ui.pop_skin();
            return ScreenCommand::Replace(Screen::game(self.skin.clone()));
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
            ui.pop_skin();
            return ScreenCommand::Quit;
        }

        ui.pop_skin();

        ScreenCommand::None
    }
}
