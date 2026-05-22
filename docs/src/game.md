# The Game Struct (`game.rs`)

The entire game world is held in one struct:

```rust
pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub pickups: Vec<AmmoPickup>,
    pub score: u32,
    pub spawn_timer: f32,
    pub events: Vec<GameEvent>,
}
```

- `new()` initializes a fresh game with the player at the center.
- `reset()` replaces the current `Game` with a brand new one (used when restarting).

All systems receive a `&mut Game` and modify its fields directly.
