### Enemy

```rust
pub struct Enemy {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub radius: f32,
    pub health: Health,
    pub damage: f32,
    pub color: Color,
    pub active: bool,
}
```

- `new(position)` – constructs an enemy with default values from config.
- `update(dt, player_pos)` – moves toward the player using simple vector math. Velocity is recomputed each frame.
- `draw()` – draws a colored circle.

Enemies have no invincibility frames; they are destroyed when `health.current <= 0`.
