### Ammo Pickup

```rust
pub struct AmmoPickup {
    pub position: Vec2,
    pub radius: f32,
    pub amount: u32,
    pub active: bool,
    pub lifetime: f32,
}
```

- `new(position)` – spawns with a fixed lifetime (`PICKUP_LIFETIME`).
- `update(dt)` – decreases `lifetime`; deactivates when it runs out.
- `draw()` – renders a golden square with the word “AMMO”.

Pickups are created when an enemy dies (by the event system) and disappear when collected or expired.
