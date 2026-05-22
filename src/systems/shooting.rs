use macroquad::audio::play_sound_once;

use crate::game::Game;
use crate::input::InputState;

/// Handle player shooting.
pub fn update_shooting(game: &mut Game, dt: f32, input: &InputState) {
    if input.shoot {
        if let Some(bullet) = game.player.shoot(dt, input.aim_dir) {
            game.bullets.push(bullet);
            if let Some(res) = &game.resources {
                play_sound_once(&res.shoot_sound);
            }
        }
    }
    if input.reload {
        game.player.weapon.start_reload();
    }
}
