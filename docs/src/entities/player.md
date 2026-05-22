### Player

```rust
pub struct Player {
    pub position: Vec2,
    pub radius: f32,
    pub speed: f32,
    pub health: Health,
    pub weapon: Weapon,
    pub turret_offset: f32,
}
```

**Important behaviors**:

- `update(dt, move_dir)` – moves the player, clamps to screen, and updates `health` and `weapon` timers.
- `shoot(dt, aim_dir) -> Option<Bullet>` – delegates to `weapon.shoot()` after calculating the turret position.
- `get_turret_position(aim_dir) -> Vec2` – returns the point on the player’s edge where the turret sits, based on the aim direction.
- `draw(aim_dir)` – renders the player body, turret, and a health bar above.

**Turret visual**: a line from center to edge in the aim direction, plus a small red circle.
