### Cleanup System

```rust
pub fn cleanup_entities(game: &mut Game)
```

Removes inactive bullets, enemies, and pickups from their respective vectors using `retain`. Keeps the lists from growing endlessly.
