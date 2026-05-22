# Game State Management (`state.rs`)

The `GameState` enum has three variants:

```rust
pub enum GameState {
    Menu,
    Playing,
    Dead,
}
```

**`update_state`** function checks input and health to transition between states:

- `Menu` → `Playing` on `confirm` (Enter).
- `Playing` → `Dead` when `player.health.current <= 0.0`.
- `Dead` → `Menu` on `confirm`.

**`draw_state`** renders different UI depending on the current state (title screen, game‑over screen). The HUD is drawn separately in `ui/hud.rs`.
