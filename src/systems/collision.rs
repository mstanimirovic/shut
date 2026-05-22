use crate::event::GameEvent;
use crate::game::Game;
use macroquad::prelude::*;

/// Check collisions and push events into the queue.
pub fn check_collisions(game: &mut Game) {
    // Bullet vs Enemy
    for (bi, bullet) in game.bullets.iter().enumerate() {
        if !bullet.active {
            continue;
        }
        for (ei, enemy) in game.enemies.iter().enumerate() {
            if !enemy.active {
                continue;
            }
            if circles_collide(bullet.position, bullet.radius, enemy.position, enemy.radius) {
                game.events.push(GameEvent::BulletHitEnemy {
                    bullet_index: bi,
                    enemy_index: ei,
                });
                break; // one bullet hits only one enemy
            }
        }
    }

    // Enemy vs Player (damage on touch)
    for enemy in game.enemies.iter() {
        if !enemy.active {
            continue;
        }
        if circles_collide(
            game.player.position,
            game.player.radius,
            enemy.position,
            enemy.radius,
        ) {
            game.events.push(GameEvent::PlayerDamaged {
                amount: enemy.damage,
            });
            break; // only one damage event per frame? We'll process later.
        }
    }

    // Player vs Ammo Pickups
    for pickup in game.pickups.iter() {
        if !pickup.active {
            continue;
        }
        if circles_collide(
            game.player.position,
            game.player.radius,
            pickup.position,
            pickup.radius,
        ) {
            game.events.push(GameEvent::PlayerPickedUpAmmo {
                amount: pickup.amount,
            });
            // We'll deactivate the pickup in event processing.
        }
    }
}

/// Simple circle-circle collision check.
fn circles_collide(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    c1.distance(c2) < r1 + r2
}
