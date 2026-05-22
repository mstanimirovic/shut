/// Health component that can be attached to any entity.
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub damage_cooldown: f32, // invincibility time after being hit
    timer: f32,
}

impl Health {
    pub fn new(max: f32, cooldown: f32) -> Self {
        Self {
            current: max,
            max,
            damage_cooldown: cooldown,
            timer: 0.0,
        }
    }

    /// Try to apply damage. Returns true if the entity took damage
    pub fn take_damage(&mut self, amount: f32) -> bool {
        if self.timer <= 0.0 {
            self.current -= amount;
            self.timer = self.damage_cooldown;
            if self.current <= 0.0 {
                self.current = 0.0;
            }
            return true;
        }
        false
    }

    /// Advance the invincibility timer.
    pub fn update(&mut self, dt: f32) {
        if self.timer > 0.0 {
            self.timer -= dt;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    /// Fraction of health remaining (0.0 - 1.0).
    pub fn fraction(&self) -> f32 {
        self.current / self.max
    }
}
