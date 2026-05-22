use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Color,
    pub active: bool,
}

impl Bullet {
    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        // Deactivate if it leaves the screen
        if self.position.x < 0.0
            || self.position.x > screen_width()
            || self.position.y < 0.0
            || self.position.y > screen_height()
        {
            self.active = false;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}
