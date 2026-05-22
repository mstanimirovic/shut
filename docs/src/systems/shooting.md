### Shooting System

```rust
pub fn update_shooting(game: &mut Game, dt: f32, input: &InputState)
```

- If `input.shoot` is true, calls `player.shoot(...)` and pushes any returned bullet into `game.bullets`.
- If `input.reload` is true, calls `player.weapon.start_reload()`.

Because input is abstracted, this same code would work with a gamepad trigger.
