use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    speed: f32,
    size: Vec2,
    color: Color,
    direction: Vec2,
    pub dead: bool,
}

impl Bullet {
    pub fn new_slow() -> Self {
        let screen = vec2(screen_width(), screen_height());
        let side = (::rand::random::<f32>() * 3.99).floor();
        let random_y = ::rand::random::<f32>() * screen.y;
        let random_x = ::rand::random::<f32>() * screen.x;
        let (position, direction) = match side {
            0. => (vec2(0., random_y), vec2(1., 0.)),
            1. => (vec2(screen.x, random_y), vec2(-1., 0.)),
            2. => (vec2(random_x, 0.), vec2(0., 1.)),
            3. => (vec2(random_x, screen.y), vec2(0., -1.)),
            _ => unreachable!(),
        };

        Bullet {
            position,
            speed: 175.,
            size: (direction.abs() + 1.) * 7.,
            color: RED,
            direction,
            dead: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.direction * self.speed * dt;

        let screen = vec2(screen_width(), screen_height());
        if self.position.x < 0.
            || self.position.x > screen.x
            || self.position.y < 0.
            || self.position.y > screen.y
        {
            self.dead = true;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }
    
    pub fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}
