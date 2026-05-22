use macroquad::prelude::*;

pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub health: i32,
    pub active: bool,
    pub color: Color,
    pub damage: f32,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            speed: 150.0, // slightly slower than player
            radius: 20.0,
            health: 1,
            active: true,
            color: RED,
            damage: 25.0,
        }
    }

    pub fn update(&mut self, dt: f32, player_position: Vec2) {
        if !self.active {
            return;
        }
        let dir = player_position - self.position;
        if dir.length() > 0.01 {
            self.velocity = dir.normalize() * self.speed;
        } else {
            self.velocity = Vec2::ZERO;
        }
        self.position += self.velocity * dt;
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}
