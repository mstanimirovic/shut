use crate::game::Game;
use crate::input::InputState;
use macroquad::prelude::*;

#[derive(PartialEq, Eq)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

/// Update the game state machine.
/// Returns the new state (or same state if unchanged).
pub fn update_state(current: &mut GameState, game: &mut Game, input: &InputState) {
    match current {
        GameState::Menu => {
            if input.confirm {
                *current = GameState::Playing;
                game.reset();
            }
        }
        GameState::Playing => {
            if !game.player.health.is_alive() {
                *current = GameState::Dead;
            }
            if input.pause {
                // Optional: could go to Paused state; here we just ignore.
            }
        }
        GameState::Dead => {
            if input.confirm {
                *current = GameState::Menu;
            }
        }
    }
}

/// Draw UI elements specific to a state.
pub fn draw_state(state: &GameState, game: &Game) {
    match state {
        GameState::Menu => {
            let title = "SHUT";
            let subtitle = "Press ENTER to start";
            let title_size = 60.0;
            let sub_size = 30.0;

            let title_dim = measure_text(title, None, title_size as u16, 1.0);
            let sub_dim = measure_text(subtitle, None, sub_size as u16, 1.0);
            draw_text(
                title,
                screen_width() / 2.0 - title_dim.width / 2.0,
                screen_height() / 3.0,
                title_size,
                DARKBLUE,
            );
            draw_text(
                subtitle,
                screen_width() / 2.0 - sub_dim.width / 2.0,
                screen_height() / 2.0,
                sub_size,
                BLACK,
            );
        }
        GameState::Playing => {
            // HUD is drawn separately by systems; nothing extra here.
        }
        GameState::Dead => {
            let text = "GAME OVER";
            let info = "Press ENTER for menu";
            let size = 50.0;
            let small = 25.0;
            let dim = measure_text(text, None, size as u16, 1.0);
            let dim_small = measure_text(info, None, small as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - dim.width / 2.0,
                screen_height() / 3.0,
                size,
                RED,
            );
            draw_text(
                info,
                screen_width() / 2.0 - dim_small.width / 2.0,
                screen_height() / 2.0,
                small,
                DARKGRAY,
            );
        }
    }
}
