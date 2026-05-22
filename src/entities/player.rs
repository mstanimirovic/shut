use crate::components::health::Health;
use crate::components::weapon::Weapon;
use crate::config;
use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub radius: f32,
    pub speed: f32,
    pub health: Health,
    pub weapon: Weapon,
    // turret visuals
    pub turret_offset: f32,
}

impl Player {
    pub fn new() -> Self {
        let radius = config::PLAYER_RADIUS;
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            radius,
            speed: config::PLAYER_SPEED,
            health: Health::new(config::PLAYER_MAX_HEALTH, 0.5),
            weapon: Weapon::pistol(),
            turret_offset: radius * 0.7,
        }
    }

    /// Move player according to input. dt is delta time.
    pub fn update(&mut self, dt: f32, move_dir: Vec2) {
        let dir = move_dir.normalize_or_zero();
        self.position += dir * self.speed * dt;

        // Keep inside screen
        self.position.x = self
            .position
            .x
            .clamp(self.radius, screen_width() - self.radius);
        self.position.y = self
            .position
            .y
            .clamp(self.radius, screen_height() - self.radius);

        self.weapon.update(dt);
        self.health.update(dt);
    }

    /// Attempt to fire using the weapon. Returns a bullet if successful.
    pub fn shoot(&mut self, dt: f32, aim_dir: Vec2) -> Option<super::bullet::Bullet> {
        let turret_pos = self.get_turret_position(aim_dir);
        self.weapon.shoot(dt, turret_pos, aim_dir)
    }

    /// Position of the turret (small circle) based on aim direction.
    pub fn get_turret_position(&self, aim_dir: Vec2) -> Vec2 {
        self.position + aim_dir * self.turret_offset
    }

    /// Draw player body, turret, and health bar.
    pub fn draw(&self, aim_dir: Vec2) {
        // Main body
        draw_circle(self.position.x, self.position.y, self.radius, DARKBLUE);
        // Turret line
        let turret_pos = self.get_turret_position(aim_dir);
        draw_line(
            self.position.x,
            self.position.y,
            turret_pos.x,
            turret_pos.y,
            3.0,
            GRAY,
        );
        // Turret circle
        draw_circle(turret_pos.x, turret_pos.y, self.radius * 0.35, RED);

        // Health bar above player
        let bar_w = 50.0;
        let bar_h = 5.0;
        let x = self.position.x - bar_w / 2.0;
        let y = self.position.y - self.radius - 15.0;
        draw_rectangle(x, y, bar_w, bar_h, RED);
        draw_rectangle(x, y, bar_w * self.health.fraction(), bar_h, GREEN);
        draw_rectangle_lines(x, y, bar_w, bar_h, 1.0, WHITE);
    }
}
