### Bullet

```rust
pub struct Bullet {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Color,
    pub active: bool,
}
```

- `update(dt)` – moves the bullet. If it exits the screen, it sets `active = false`.
- `draw()` – draws a small circle.

Bullets are spawned by `weapon.shoot()`.
