### Spawner System

```rust
pub fn update_spawner(game: &mut Game, dt: f32)
```

Uses a simple timer (`game.spawn_timer`). When it reaches zero, a new enemy is placed at a random edge of the screen (using `macroquad::rand::gen_range`).

The timer is reset to `config::SPAWN_DELAY`.
