use macroquad::audio::play_sound_once;

use crate::event::GameEvent;
use crate::game::Game;

/// Process all events accumulated during this frame.
pub fn process_events(game: &mut Game) {
    let events: Vec<_> = game.events.drain(..).collect();
    for event in events {
        match event {
            GameEvent::BulletHitEnemy {
                bullet_index,
                enemy_index,
            } => {
                // Deactivate bullet
                if let Some(bullet) = game.bullets.get_mut(bullet_index) {
                    bullet.active = false;
                }
                // Damage enemy (we don't store bullet damage, use 1 for now)
                if let Some(enemy) = game.enemies.get_mut(enemy_index) {
                    enemy.health.take_damage(1.0);
                    if !enemy.health.is_alive() {
                        enemy.active = false;
                        game.score += 100;
                        // Spawn an ammo pickup at the enemy's death position
                        game.pickups
                            .push(crate::entities::pickup::AmmoPickup::new(enemy.position));
                    }
                }
            }
            GameEvent::EnemyDied { position } => {
                // Could spawn visual effects here; already handled in bullet hit.
                _ = position;
            }
            GameEvent::PlayerDamaged { amount } => {
                let took = game.player.health.take_damage(amount);

                if let Some(res) = &game.resources
                    && took
                {
                    play_sound_once(&res.hit_sound);
                }
            }
            GameEvent::PlayerPickedUpAmmo { amount } => {
                game.player.weapon.add_ammo(amount);
                // Deactivate all pickups that are colliding (simplification: just deactivate all active ones)
                for pickup in game.pickups.iter_mut() {
                    if pickup.active {
                        pickup.active = false;
                    }
                }
            }
            GameEvent::WeaponReloaded => {
                // Could play sound; nothing else needed.
            }
        }
    }
}
