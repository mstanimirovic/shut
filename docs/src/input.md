# Input System (`input.rs`)

The `InputState` struct abstracts all player actions away from physical devices.
The game logic never asks “is the A key pressed?” – instead it reads `input.move_dir` or `input.shoot`.

```rust
pub struct InputState {
    pub move_dir: Vec2,    // normalised or zero
    pub aim_dir: Vec2,     // direction from player to mouse
    pub shoot: bool,       // true while button held
    pub reload: bool,      // true only on the frame R is pressed
    pub confirm: bool,
    pub pause: bool,
}
```

**Construction functions**:

- `from_keyboard_and_mouse(player_pos: Vec2) -> Self` – builds state from keyboard & mouse.
- `from_touch(...)` – placeholder for future mobile support.

This makes it trivial to add AI-controlled players or gamepad support later.
