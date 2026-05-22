### Collision System

```rust
pub fn check_collisions(game: &mut Game)
```

Iterates over bullets and enemies, and over enemies and the player, using simple circle‑circle distance checks.

On overlap, it pushes the appropriate `GameEvent` variants into `game.events`. It **does not** modify health or score directly.

This is the only system that creates events.
