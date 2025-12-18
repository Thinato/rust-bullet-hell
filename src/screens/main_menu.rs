use macroquad::{
    math::vec2,
    prelude::{screen_height, screen_width},
    time::draw_fps,
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

        let screen = vec2(screen_width(), screen_height());
        let button_size = vec2(120., 30.);
        let spacing = 12.;
        let column_height = 3. * button_size.y + 2. * spacing;
        let start = vec2(
            (screen.x - button_size.x) * 0.5,
            (screen.y - column_height) * 0.5,
        );

        if widgets::Button::new("START GAME")
            .position(start)
            .size(button_size)
            .ui(ui)
        {
            ui.pop_skin();
            return ScreenCommand::Replace(Screen::game(self.skin.clone()));
        }

        if widgets::Button::new("OPTIONS")
            .position(vec2(start.x, start.y + button_size.y + spacing))
            .size(button_size)
            .ui(ui)
        {
            println!("Options clicked (not implemented yet)");
        }

        if widgets::Button::new("EXIT")
            .position(vec2(start.x, start.y + 2. * button_size.y + 2. * spacing))
            .size(button_size)
            .ui(ui)
        {
            ui.pop_skin();
            return ScreenCommand::Quit;
        }

        ui.pop_skin();

        ScreenCommand::None
    }
}
