use crate::config;
use crate::entities::enemy::Enemy;
use crate::game::Game;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

/// Spawn enemies periodically from screen edges.
pub fn update_spawner(game: &mut Game, dt: f32) {
    game.spawn_timer -= dt;
    if game.spawn_timer <= 0.0 {
        game.spawn_timer = config::SPAWN_DELAY;
        let side = gen_range(0, 4);
        let pos = match side {
            0 => vec2(-30.0, gen_range(0.0, screen_height())),
            1 => vec2(screen_width() + 30.0, gen_range(0.0, screen_height())),
            2 => vec2(gen_range(0.0, screen_width()), -30.0),
            _ => vec2(gen_range(0.0, screen_width()), screen_height() + 30.0),
        };
        game.enemies.push(Enemy::new(pos));
    }
}
