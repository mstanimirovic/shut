use macroquad::prelude::*;

use crate::bullet::Bullet;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub turret_offset: f32,
    pub texture: Option<Texture2D>,
    pub shoot_cooldown: f32,
    shoot_timer: f32,
    pub health: f32,
    pub max_health: f32,
    /// duration of invulnerability after taking damage
    damage_cooldown: f32,
    damage_timer: f32,
}

impl Player {
    pub fn new(texture: Option<Texture2D>) -> Self {
        Self {
            position: vec2(screen_width() / 2.0, screen_height() / 2.0),
            speed: 300.0,
            radius: 30.0,
            turret_offset: 20.0,
            texture,
            shoot_cooldown: 0.2,
            shoot_timer: 0.0,
            health: 100.0,
            max_health: 100.0,
            damage_cooldown: 0.5, // half second invulnerability
            damage_timer: 0.0,
        }
    }

    /// Apply damage to player, only if cooldown has elapsed.
    /// Returns true if player is still alive.
    pub fn take_damage(&mut self, amount: f32) -> bool {
        if self.damage_timer <= 0.0 {
            self.health -= amount;
            self.damage_timer = self.damage_cooldown;
            if self.health <= 0.0 {
                self.health = 0.0;
                return false; // player is dead
            }
        }
        true
    }

    /// Called each frame to countdown invulnerability timer.
    pub fn update_timers(&mut self, dt: f32) {
        if self.damage_timer > 0.0 {
            self.damage_timer -= dt;
        }
    }

    pub fn get_turret_position(&self) -> Vec2 {
        let mouse_pos: Vec2 = mouse_position().into();
        let dir = mouse_pos - self.position;
        // If direction is zero (mouse at center), default to right
        if dir.length() < 0.001 {
            return self.position + vec2(self.turret_offset, 0.0);
        }
        let angle = dir.y.atan2(dir.x);
        self.position + Vec2::new(angle.cos(), angle.sin()) * self.turret_offset
    }

    pub fn shoot(&mut self, dt: f32) -> Option<Bullet> {
        self.shoot_timer -= dt;
        if self.shoot_timer <= 0.0 && is_mouse_button_down(MouseButton::Left) {
            self.shoot_timer = self.shoot_cooldown;
            let turret_pos = self.get_turret_position();
            let mouse_pos: Vec2 = mouse_position().into();
            let dir = (mouse_pos - self.position).normalize_or_zero();
            let bullet_speed = 500.0;
            Some(Bullet {
                position: turret_pos,
                velocity: dir * bullet_speed,
                active: true,
            })
        } else {
            None
        }
    }

    pub fn update(&mut self, dt: f32, move_dir: Vec2) {
        let dir = move_dir.normalize_or_zero();
        self.position += dir * self.speed * dt;

        // Clamp position to screen bounds
        self.position.x = self
            .position
            .x
            .clamp(self.radius, screen_width() - self.radius);
        self.position.y = self
            .position
            .y
            .clamp(self.radius, screen_height() - self.radius);
    }

    pub fn draw(&self) {
        // Draw player body and turret
        draw_circle(self.position.x, self.position.y, self.radius, DARKBLUE);
        let turret_pos = self.get_turret_position();
        draw_line(
            self.position.x,
            self.position.y,
            turret_pos.x,
            turret_pos.y,
            4.0,
            GRAY,
        );
        draw_circle(turret_pos.x, turret_pos.y, 10.0, RED);

        // Health bar
        let bar_width = 60.0;
        let bar_height = 6.0;
        let bar_x = self.position.x - bar_width / 2.0;
        let bar_y = self.position.y - self.radius - 15.0;
        let fill_width = bar_width * (self.health / self.max_health);

        // Background (red)
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, RED);
        // Fill (green)
        draw_rectangle(bar_x, bar_y, fill_width, bar_height, GREEN);
        // Border (white)
        draw_rectangle_lines(
            bar_x - 2.0,
            bar_y - 2.0,
            bar_width + 4.0,
            bar_height + 4.0,
            2.0,
            WHITE,
        );
    }
}
