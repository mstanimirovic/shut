#![allow(unused)]

mod config;
mod event;
mod game;
mod input;
mod resources;
mod state;
mod components {
    pub mod health;
    pub mod weapon;
}
mod entities {
    pub mod bullet;
    pub mod enemy;
    pub mod pickup;
    pub mod player;
}
mod systems {
    pub mod cleanup;
    pub mod collision;
    pub mod events;
    pub mod movement;
    pub mod shooting;
    pub mod spawner;
}
mod ui {
    pub mod hud;
}
// mod resources; // not used, but placeholder

use game::Game;
use input::InputState;
use macroquad::prelude::*;
use state::{GameState, draw_state, update_state};

#[macroquad::main("shut")]
async fn main() {
    set_pc_assets_folder("assets");
    let mut game = Game::new().await;
    let mut state = GameState::Menu;

    loop {
        let dt = get_frame_time();
        clear_background(LIGHTGRAY);

        // Build input (keyboard/mouse)
        let input = InputState::from_keyboard_and_mouse(game.player.position);

        // State machine (may change state)
        update_state(&mut state, &mut game, &input);

        match state {
            GameState::Playing => {
                // Run all systems in order
                systems::movement::update_movement(&mut game, dt, &input);
                systems::shooting::update_shooting(&mut game, dt, &input);
                systems::spawner::update_spawner(&mut game, dt);
                systems::collision::check_collisions(&mut game);
                systems::events::process_events(&mut game);
                systems::cleanup::cleanup_entities(&mut game);

                // Render everything
                for bullet in game.bullets.iter() {
                    bullet.draw();
                }
                for enemy in game.enemies.iter() {
                    enemy.draw();
                }
                for pickup in game.pickups.iter() {
                    pickup.draw();
                }
                // Player drawn after enemies so health bar is visible?
                game.player.draw(input.aim_dir);
            }
            _ => {}
        }

        // UI
        if state == GameState::Playing || state == GameState::Dead {
            ui::hud::draw_hud(&game);
        }
        draw_state(&state, &game);

        next_frame().await;
    }
}
