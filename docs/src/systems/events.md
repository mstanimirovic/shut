### Event Processing System

```rust
pub fn process_events(game: &mut Game)
```

Drains `game.events` and handles each one:

- `BulletHitEnemy` – deactivates bullet, damages enemy. If enemy dies, deactivates it, increases score, and spawns an `AmmoPickup` at the death position.
- `EnemyDied` – currently unused, but a perfect place to add death effects.
- `PlayerDamaged` – applies damage to player via `player.health.take_damage()`.
- `PlayerPickedUpAmmo` – adds ammo to player’s reserve and deactivates the collided pickup.

This centralization means that adding score multipliers, achievements, or screen shake only requires changing this file.
