use macroquad::{
    color::{Color},
    text::load_ttf_font,
    ui::{Skin, Ui, root_ui},
};

use crate::screens::{Screen, ScreenCommand};

pub struct Director {
    current_screen: Screen,
}

impl Director {
    pub async fn new() -> Self {
        let skin = Self::build_default_skin().await;

        Director {
            current_screen: Screen::main_menu(skin.clone()),
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        self.current_screen.update(dt)
    }

    pub fn draw(&mut self) -> ScreenCommand {
        let mut ui_handle = root_ui();
        let ui: &mut Ui = &mut *ui_handle;

        self.current_screen.draw_with_ui(ui)
    }

    pub fn change_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    async fn build_default_skin() -> Skin {
        let font = load_ttf_font("assets/fonts/8bit.ttf")
            .await
            .expect("failed to load assets/fonts/8bit.ttf");

        let button_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .color(Color::from_rgba(0, 0, 0, 255))
            .color_hovered(Color::from_rgba(0, 0, 0, 255))
            .color_clicked(Color::from_rgba(0, 0, 0, 255))
            .text_color(Color::from_rgba(255, 255, 255, 255))
            .text_color_hovered(Color::from_rgba(224, 140, 30, 255))
            .text_color_clicked(Color::from_rgba(235, 154, 49, 255))
            .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    }
}
