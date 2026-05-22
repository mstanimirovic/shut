# Configuration (`config.rs`)

All tunable constants live here. Changing numbers in this file is enough to rebalance the entire game.

```rust
# Important constants
pub const PLAYER_SPEED: f32 = 300.0;
pub const PLAYER_RADIUS: f32 = 25.0;
pub const PLAYER_MAX_HEALTH: f32 = 100.0;

pub const ENEMY_SPEED: f32 = 120.0;
pub const ENEMY_RADIUS: f32 = 18.0;
pub const ENEMY_HEALTH: i32 = 1;
pub const ENEMY_DAMAGE: f32 = 20.0;

pub const BULLET_SPEED: f32 = 600.0;
pub const BULLET_RADIUS: f32 = 4.0;

// Weapon parameters
pub const WEAPON_PISTOL_MAGAZINE: u32 = 12;
pub const WEAPON_PISTOL_RESERVE: u32 = 36;
pub const WEAPON_PISTOL_RELOAD_TIME: f32 = 1.5;
pub const WEAPON_PISTOL_COOLDOWN: f32 = 0.15;

// Spawning & pickups
pub const SPAWN_DELAY: f32 = 2.0;
pub const AMMO_PICKUP_AMOUNT: u32 = 6;
pub const PICKUP_RADIUS: f32 = 12.0;
pub const PICKUP_LIFETIME: f32 = 10.0;
```

**Design note**: Because all systems read these constants directly (they are imported from `crate::config`), adjusting difficulty is a one‑line change.
