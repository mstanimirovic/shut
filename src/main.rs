#![allow(unused)]

mod bullet;
mod enemy;
mod player;
mod state;

use macroquad::{miniquad::window::quit, prelude::*};

use crate::{bullet::Bullet, enemy::Enemy, player::Player, state::GameState};

fn circles_collide(c1: Vec2, r1: f32, c2: Vec2, r2: f32) -> bool {
    c1.distance(c2) < r1 + r2
}

fn get_keyboard_move_dir() -> Vec2 {
    let mut dir = Vec2::ZERO;
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        dir.x += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        dir.x -= 1.0;
    }
    if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
        dir.y += 1.0;
    }
    if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
        dir.y -= 1.0;
    }
    dir
}

fn spawn_enemy(spawn_timer: &mut f32, spawn_delay: f32, enemies: &mut Vec<Enemy>) {
    *spawn_timer -= get_frame_time();
    if *spawn_timer <= 0.0 {
        *spawn_timer = spawn_delay;
        let side = rand::gen_range(0, 4);
        let pos = match side {
            0 => vec2(-20.0, rand::gen_range(0.0, screen_height())),
            1 => vec2(screen_width() + 20.0, rand::gen_range(0.0, screen_height())),
            2 => vec2(rand::gen_range(0.0, screen_width()), -20.0),
            _ => vec2(rand::gen_range(0.0, screen_width()), screen_height() + 20.0),
        };
        enemies.push(Enemy::new(pos));
    }
}

fn update_all_enemies(enemies: &mut Vec<Enemy>, player_position: Vec2, dt: f32) {
    for enemy in enemies.iter_mut() {
        enemy.update(dt, player_position);
    }
    enemies.retain(|e| e.active);
}

fn update_all_bullets(bullets: &mut Vec<Bullet>, dt: f32) {
    for bullet in bullets.iter_mut() {
        bullet.update(dt);
    }
    bullets.retain(|b| b.active);
}

fn check_bullet_enemy_collisions(bullets: &mut Vec<Bullet>, enemies: &mut Vec<Enemy>) {
    for bullet in bullets.iter_mut() {
        if !bullet.active {
            continue;
        }
        for enemy in enemies.iter_mut() {
            if !enemy.active {
                continue;
            }
            if circles_collide(bullet.position, 5.0, enemy.position, enemy.radius) {
                bullet.active = false;
                enemy.health -= 1;
                if enemy.health <= 0 {
                    enemy.active = false;
                }
                break;
            }
        }
    }
}

fn check_player_collision(player: &mut Player, enemies: &mut Vec<Enemy>) {
    for enemy in enemies.iter_mut() {
        if !enemy.active {
            continue;
        }
        if circles_collide(player.position, player.radius, enemy.position, enemy.radius) {
            if !player.take_damage(enemy.damage) {
                println!("Game Over!");
            }
        }
    }
}

fn draw_all_entities(player: &Player, bullets: &[Bullet], enemies: &[Enemy]) {
    player.draw();
    for bullet in bullets.iter() {
        bullet.draw();
    }
    for enemy in enemies.iter() {
        enemy.draw();
    }
}

fn draw_ui(enemies_count: usize) {
    draw_text(
        &format!("Enemies: {}", enemies_count),
        20.0,
        20.0,
        26.0,
        BLACK,
    );
}

#[macroquad::main("shut")]
async fn main() {
    let mut game_state = GameState::Menu;
    let mut player = Player::new(None);
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies = Vec::new();

    let mut spawn_timer = 0.0;
    let spawn_delay = 1.0;

    loop {
        let dt = get_frame_time();
        clear_background(GREEN);

        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Escape) {
                    quit();
                }
                draw_text(
                    "SHUT - Pritisnite ENTER za početak",
                    screen_width() / 2.0 - 250.0,
                    screen_height() / 2.0,
                    40.0,
                    BLACK,
                );
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Playing;
                    player = Player::new(None);
                    bullets.clear();
                    enemies.clear();
                    spawn_timer = 0.0;
                }
            }
            GameState::Playing => {
                let move_dir = get_keyboard_move_dir();
                player.update(dt, move_dir);
                player.update_timers(dt);

                if let Some(bullet) = player.shoot(dt) {
                    bullets.push(bullet);
                }

                spawn_enemy(&mut spawn_timer, spawn_delay, &mut enemies);

                update_all_enemies(&mut enemies, player.position, dt);
                update_all_bullets(&mut bullets, dt);

                check_bullet_enemy_collisions(&mut bullets, &mut enemies);
                check_player_collision(&mut player, &mut enemies);

                if player.health <= 0.0 {
                    game_state = GameState::Dead;
                }
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Menu;
                }

                draw_all_entities(&player, &bullets, &enemies);
                draw_ui(enemies.len());
            }
            GameState::Dead => {
                draw_text(
                    "GAME OVER - Press R to restart the game",
                    screen_width() / 2.0 - 320.0,
                    screen_height() / 2.0,
                    40.0,
                    RED,
                );
                draw_text(
                    "Press M for menu",
                    screen_width() / 2.0 - 220.0,
                    screen_height() / 2.0 + 60.0,
                    30.0,
                    DARKGRAY,
                );
                if is_key_pressed(KeyCode::Escape) {
                    quit();
                }
                if is_key_pressed(KeyCode::R) {
                    game_state = GameState::Playing;
                    player = Player::new(None);
                    bullets.clear();
                    enemies.clear();
                    spawn_timer = 0.0;
                }
                if is_key_pressed(KeyCode::M) {
                    game_state = GameState::Menu;
                }
            }
        }

        next_frame().await;
    }
}
