# Event System (`event.rs`)

Events are small enums that represent things that happened during the frame.
Systems push them into `game.events`, and later the `process_events` system handles them.

```rust
pub enum GameEvent {
    BulletHitEnemy { bullet_index: usize, enemy_index: usize },
    EnemyDied { position: Vec2 },
    PlayerDamaged { amount: f32 },
    PlayerPickedUpAmmo { amount: u32 },
    WeaponReloaded,
}
```

**Why events?** Without them, collision code would need to directly modify health, score, and spawn pickups – mixing responsibilities. With events, the collision system only detects overlaps and emits events. The event processing system decides what to do, keeping each system focused.
