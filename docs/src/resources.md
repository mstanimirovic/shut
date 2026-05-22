# Resources & Assets (`resources.rs`)

*Currently a placeholder module.*
In a full project you would add a `Resources` struct that loads and holds textures and sounds:

```rust
pub struct Resources {
    pub player_texture: Texture2D,
    pub shoot_sound: Sound,
    // ...
}
```

Load it once in `main` with:

```rust
let resources = Resources::new().await;
```

Then pass it to drawing systems or the player’s draw method.
This prevents loading the same file multiple times and makes asset hot‑reloading easier to add later.
