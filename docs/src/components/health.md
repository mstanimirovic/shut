### Health Component

`Health` is a reusable component for any entity that can take damage.

```rust
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub damage_cooldown: f32,  // invulnerability window after being hit
    timer: f32,
}

impl Health {
    pub fn new(max: f32, cooldown: f32) -> Self { ... }
    pub fn take_damage(&mut self, amount: f32) -> bool { ... }
    pub fn update(&mut self, dt: f32) { ... }
    pub fn is_alive(&self) -> bool { ... }
    pub fn fraction(&self) -> f32 { ... }
}
```

- `take_damage` applies damage only if the invincibility timer has run out and returns `false` if health drops to zero.
- `update` decrements the timer every frame.
- `fraction` is used by the HUD for health bars.

Player has a cooldown (0.5s) to avoid instant death; enemies have 0.0 because they die in one hit anyway.
