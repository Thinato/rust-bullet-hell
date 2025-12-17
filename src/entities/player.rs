use macroquad::prelude::*;

pub struct Player {
    position: Vec2,
    speed: f32,
    size: f32,
    color: Color,
    direction: Vec2,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: vec2(100., 100.),
            speed: 100.,
            size: 10.0,
            color: GREEN,
            direction: Vec2::ZERO,
        }
    }

    pub async fn draw(&mut self) {
        draw_circle(self.position.x, self.position.y, self.size, self.color);
    }

    pub fn go(&mut self, dt: f32) {
        if self.direction.length() == 0.0 {
            return;
        }
        let norm = self.direction.normalize();
        
        let velocity = norm * self.speed * dt;
        
        println!("Velocity: {:?}", velocity);
        
        self.position += velocity;
    }
    
    pub fn add_force(&mut self, force: Vec2) {
        self.direction += force;
    }
    pub fn set_direction(&mut self, direction: Vec2) {
        self.direction = direction;
    }
}
