use crate::game::Game;
use macroquad::prelude::*;

/// Draw the heads-up display at the bottom of the screen.
pub fn draw_hud(game: &Game) {
    let mut y_base = screen_height() - 30.0;
    let font_size = 24.0;

    // Background panel
    // draw_rectangle(
    //     0.0,
    //     y_base,
    //     screen_width(),
    //     70.0,
    //     Color::new(0.0, 0.0, 0.0, 0.5),
    // );

    // Health
    let health_text = format!(
        "HP: {:.0}/{:.0}",
        game.player.health.current, game.player.health.max
    );
    draw_text(&health_text, 20.0, y_base, font_size, GREEN);

    // Ammo
    let ammo_text = game.player.weapon.ammo_text();
    draw_text(&ammo_text, 170.0, y_base, font_size, YELLOW);

    // Enemies count
    let enemies_text = format!("Enemies: {}", game.enemies.len());
    draw_text(&enemies_text, 470.0, y_base, font_size, RED);

    // Score
    let score_text = format!("Score: {}", game.score);
    draw_text(&score_text, 670.0, y_base, font_size, WHITE);
}
