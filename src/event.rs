use macroquad::prelude::*;

/// Events that can happen in the game world.
/// Systems push these events into a queue, and later they are processed.
pub enum GameEvent {
    BulletHitEnemy {
        bullet_index: usize,
        enemy_index: usize,
    },
    EnemyDied {
        position: Vec2,
    },
    PlayerDamaged {
        amount: f32,
    },
    PlayerPickedUpAmmo {
        amount: u32,
    },
    WeaponReloaded,
}
