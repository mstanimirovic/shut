### Movement System

```rust
pub fn update_movement(game: &mut Game, dt: f32, input: &InputState)
```

Calls `update` on the player, every enemy, every bullet, and every pickup.
It does **not** handle input – the player receives `input.move_dir` which was already computed.

This system is simple but essential: it advances all positions.
