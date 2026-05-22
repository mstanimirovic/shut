use crate::game::Game;

/// Remove inactive entities.
pub fn cleanup_entities(game: &mut Game) {
    game.bullets.retain(|b| b.active);
    game.enemies.retain(|e| e.active);
    game.pickups.retain(|p| p.active);
}
