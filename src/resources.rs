use macroquad::audio::{Sound, load_sound};
use macroquad::prelude::*;

pub struct Resources {
    // pub player_texture: Texture2D,
    // pub enemy_texture: Texture2D,
    pub shoot_sound: Sound,
    pub hit_sound: Sound,
}

impl Resources {
    pub async fn new() -> Result<Self, macroquad::Error> {
        // let player_texture = load_texture("player.png").await?;
        // let enemy_texture = load_texture("enemy.png").await?;
        let shoot_sound = load_sound("shoot.wav").await?;
        let hit_sound = load_sound("hit.wav").await?;

        Ok(Self {
            // player_texture,
            // enemy_texture,
            shoot_sound,
            hit_sound,
        })
    }
}
