use crate::config;
use crate::entities::bullet::Bullet;
use macroquad::prelude::*;

/// Weapon component: manages ammo, cooldown, reload.
pub struct Weapon {
    pub name: String,
    pub magazine_ammo: u32,
    pub reserve_ammo: u32,
    pub max_magazine: u32,
    pub reload_time: f32,
    pub cooldown: f32,
    pub bullet_speed: f32,
    pub bullet_radius: f32,
    pub bullet_color: Color,
    reload_timer: f32,
    shoot_timer: f32,
    pub is_reloading: bool,
}

impl Weapon {
    /// Create the default pistol.
    pub fn pistol() -> Self {
        Self {
            name: "Pistol".into(),
            magazine_ammo: config::WEAPON_PISTOL_MAGAZINE,
            reserve_ammo: config::WEAPON_PISTOL_RESERVE,
            max_magazine: config::WEAPON_PISTOL_MAGAZINE,
            reload_time: config::WEAPON_PISTOL_RELOAD_TIME,
            cooldown: config::WEAPON_PISTOL_COOLDOWN,
            bullet_speed: config::BULLET_SPEED,
            bullet_radius: config::BULLET_RADIUS,
            bullet_color: YELLOW,
            reload_timer: 0.0,
            shoot_timer: 0.0,
            is_reloading: false,
        }
    }

    /// Attempt to fire a bullet. Returns Some(Bullet) if successful.
    /// `origin` – world position where the bullet spawns.
    /// `direction` – normalized firing direction.
    pub fn shoot(&mut self, dt: f32, origin: Vec2, direction: Vec2) -> Option<Bullet> {
        if self.is_reloading {
            return None;
        }
        self.shoot_timer -= dt;
        if self.shoot_timer <= 0.0 {
            if self.magazine_ammo == 0 {
                return None;
            }
            self.shoot_timer = self.cooldown;
            self.magazine_ammo -= 1;
            Some(Bullet {
                position: origin,
                velocity: direction * self.bullet_speed,
                radius: self.bullet_radius,
                color: self.bullet_color,
                active: true,
            })
        } else {
            None
        }
    }

    /// Start reloading if possible.
    pub fn start_reload(&mut self) {
        if self.is_reloading || self.magazine_ammo == self.max_magazine || self.reserve_ammo == 0 {
            return;
        }
        self.is_reloading = true;
        self.reload_timer = self.reload_time;
    }

    /// Update reload progress. Should be called every frame.
    pub fn update(&mut self, dt: f32) {
        if self.is_reloading {
            self.reload_timer -= dt;
            if self.reload_timer <= 0.0 {
                let needed = self.max_magazine - self.magazine_ammo;
                let transfer = needed.min(self.reserve_ammo);
                self.magazine_ammo += transfer;
                self.reserve_ammo -= transfer;
                self.is_reloading = false;
                self.reload_timer = 0.0;
            }
        }
    }

    /// Add ammo to reserve (from pickups).
    pub fn add_ammo(&mut self, amount: u32) {
        self.reserve_ammo += amount;
    }

    /// Text representation for UI.
    pub fn ammo_text(&self) -> String {
        if self.is_reloading {
            format!("Reloading... {:.1}s", self.reload_timer)
        } else {
            format!(
                "{} / {}  ({} reserve)",
                self.magazine_ammo, self.max_magazine, self.reserve_ammo
            )
        }
    }
}
