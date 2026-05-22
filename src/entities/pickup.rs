use crate::config;
use macroquad::prelude::*;

pub struct AmmoPickup {
    pub position: Vec2,
    pub radius: f32,
    pub amount: u32,
    pub active: bool,
    pub lifetime: f32, // remaining time before disappearing
}

impl AmmoPickup {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            radius: config::PICKUP_RADIUS,
            amount: config::AMMO_PICKUP_AMOUNT,
            active: true,
            lifetime: config::PICKUP_LIFETIME,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.active = false;
        }
    }

    pub fn draw(&self) {
        if !self.active {
            return;
        }
        // Draw an ammo box (square with bullets symbol)
        draw_rectangle(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
            GOLD,
        );
        // Simple bullet icon
        draw_rectangle(
            self.position.x - self.radius * 0.3,
            self.position.y - self.radius * 0.6,
            self.radius * 0.6,
            self.radius * 1.2,
            BROWN,
        );
        // Text "AMMO"
        draw_text(
            "AMMO",
            self.position.x - 20.0,
            self.position.y - 5.0,
            15.0,
            BLACK,
        );
    }
}
