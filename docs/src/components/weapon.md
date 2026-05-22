### Weapon Component

The `Weapon` struct handles everything related to firing bullets, ammo management, and reloading.

```rust
pub struct Weapon {
    pub name: String,
    pub magazine_ammo: u32,
    pub reserve_ammo: u32,
    pub max_magazine: u32,
    pub reload_time: f32,
    pub cooldown: f32,
    pub bullet_speed: f32,
    pub bullet_radius: f32,
    pub bullet_color: Color,
    reload_timer: f32,
    shoot_timer: f32,
    pub is_reloading: bool,
}
```

**Key methods**:

- `shoot(dt, origin, direction) -> Option<Bullet>` – consumes one ammo if cooldown allows, returns a new bullet.
- `start_reload()` – begins reload if magazine isn’t full and reserve ammo exists.
- `update(dt)` – called every frame; progresses reload timer and refills magazine when done.
- `add_ammo(amount)` – increases reserve ammo (used by pickups).

This component can be attached to the player, enemies, or turrets. The default pistol is created via `Weapon::pistol()` which reads constants from `config.rs`.
