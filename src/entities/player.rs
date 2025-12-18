use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    size: f32,
    speed: f32,
    color: Color,
    immune_color: Color,
    direction: Vec2,
    pub health: i32,
    immunity: bool,
    immunity_timer: f32,
}

impl Player {
    pub fn new_at(position: Vec2) -> Self {
        Player {
            position,
            speed: 100.,
            size: 20.0,
            color: GREEN,
            immune_color: DARKGREEN,
            direction: Vec2::ZERO,
            health: 100,
            immunity: false,
            immunity_timer: 0.0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size,
            self.size,
            if self.immunity { self.immune_color } else { self.color },
        );
    }
    
    pub fn update(&mut self, dt: f32) {
        if self.immunity {
            self.immunity_timer -= dt;
            if self.immunity_timer <= 0.0 {
                self.immunity = false;
            }
        }
        if self.direction.length() == 0.0 {
            return;
        }
        let norm = self.direction.normalize();

        let velocity = norm * self.speed * dt;

        self.position += velocity;
    }


    pub fn set_direction(&mut self, direction: Vec2) {
        self.direction = direction;
    }

    pub fn take_damage(&mut self, damage: i32) {
        if self.immunity {
            return;
        }
        println!("Player took damage!");
        self.immunity = true;
        self.immunity_timer = 0.5;
        self.health -= damage;
        if self.health <= 0 {
            self.health = 0;
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size, self.size)
    }
}
