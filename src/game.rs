use crate::screens::ScreenCommand;
use crate::screens::director::Director;
use macroquad::prelude::*;

pub struct Game {
    director: Director,
}

impl Game {
    pub async fn new() -> Self {
        Game {
            director: Director::new().await,
        }
    }

    pub async fn start(&mut self) {
        loop {
            let dt = get_frame_time();

            let update_command = self.director.update(dt);
            if self.apply_command(update_command) {
                break;
            }

            clear_background(BLACK);

            let draw_command = self.director.draw();
            if self.apply_command(draw_command) {
                break;
            }
            next_frame().await
        }
    }

    fn apply_command(&mut self, command: ScreenCommand) -> bool {
        match command {
            ScreenCommand::None => false,
            ScreenCommand::Replace(screen) => {
                self.director.change_screen(screen);
                false
            }
            ScreenCommand::Quit => true,
        }
    }
}
