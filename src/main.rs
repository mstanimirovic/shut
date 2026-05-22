#![allow(unused)]

use macroquad::prelude::*;

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}

const MOVEMENT_SPEED: f32 = 200.0;

#[macroquad::main("SHUT")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 16.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        collided: false,
    };
    let mut bullets: Vec<Shape> = vec![];
    let mut gameover = false;
    loop {
        clear_background(DARKPURPLE);
        let dt = get_frame_time();

        if !gameover {
            // check player inputs
            if is_key_down(KeyCode::Right) {
                circle.x += MOVEMENT_SPEED * dt;
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= MOVEMENT_SPEED * dt;
            }
            if is_key_down(KeyCode::Down) {
                circle.y += MOVEMENT_SPEED * dt;
            }
            if is_key_down(KeyCode::Up) {
                circle.y -= MOVEMENT_SPEED * dt;
            }
            circle.x = clamp(circle.x, circle.size, screen_width() - circle.size);
            circle.y = clamp(circle.y, circle.size, screen_height() - circle.size);

            if is_key_pressed(KeyCode::Space) {
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y,
                    speed: circle.speed * 2.0,
                    size: 5.0,
                    collided: false,
                });
            }

            // rand generate an enemy
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    collided: false,
                });
            }

            // update gravity for enemies
            for square in &mut squares {
                square.y += square.speed * dt;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * dt;
            }

            // remove all enemies that are off the screen
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > 0.0 - bullet.size / 2.0);

            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                    }
                }
            }
        }

        if squares.iter().any(|square| circle.collides_with(square)) {
            gameover = true;
        }

        if gameover && is_key_pressed(KeyCode::Space) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
        }

        // draw the enemies
        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        // draw the player
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimensions = measure_text(text, None, 50, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        next_frame().await
    }
}
