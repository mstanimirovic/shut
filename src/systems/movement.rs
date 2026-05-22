use crate::game::Game;
use crate::input::InputState;

/// Update player and enemy positions based on input and AI.
pub fn update_movement(game: &mut Game, dt: f32, input: &InputState) {
    game.player.update(dt, input.move_dir);

    for enemy in game.enemies.iter_mut() {
        enemy.update(dt, game.player.position);
    }

    for bullet in game.bullets.iter_mut() {
        bullet.update(dt);
    }

    for pickup in game.pickups.iter_mut() {
        pickup.update(dt);
    }
}
