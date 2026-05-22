use crate::components::health::Health;
use crate::config;
use macroquad::prelude::*;

pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub health: Health,
    pub damage: f32,
    pub color: Color,
    pub active: bool,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            speed: config::ENEMY_SPEED,
            radius: config::ENEMY_RADIUS,
            health: Health::new(config::ENEMY_HEALTH as f32, 0.0), // enemies have no invincibility
            damage: config::ENEMY_DAMAGE,
            color: RED,
            active: true,
        }
    }

    /// Move toward player position.
    pub fn update(&mut self, dt: f32, player_pos: Vec2) {
        if !self.active {
            return;
        }
        let dir = player_pos - self.position;
        if dir.length() > 0.01 {
            self.velocity = dir.normalize() * self.speed;
        } else {
            self.velocity = Vec2::ZERO;
        }
        self.position += self.velocity * dt;
        self.health.update(dt);
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}
