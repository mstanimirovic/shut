# shut – a 2D top-down shooter

A fast-paced arena shooter written in Rust with [Macroquad](https://macroquad.rs/).  
Survive waves of enemies, manage your ammo, and collect pickups to stay alive.

## Features

- Smooth WASD movement & mouse aiming
- Weapon with magazine, reserve ammo, and manual reload (`R`)
- Enemies that chase the player and deal damage on contact
- Ammo pickups dropped by killed enemies
- Health bar, ammo counter, score, and enemy count on HUD
- Three game states: Menu, Playing, Dead

## How to Run

### Desktop (native)

```bash
cargo run --release
```

Place your assets (textures, sounds) in the `assets/` folder.  
The game also works without any external assets – all graphics are geometry fallbacks.

### Web (WASM)

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Then serve the `target/wasm32-unknown-unknown/release/` directory with a web server.

## Controls

| Action            | Keyboard/Mouse        |
| ----------------- | --------------------- |
| Move              | `WASD` / Arrow keys   |
| Aim               | Mouse                 |
| Shoot             | Left mouse button     |
| Reload            | `R`                   |
| Confirm (menu)    | `Enter`               |

## Architecture

The code follows a modular, event-driven design:

- `components/` – reusable data (Health, Weapon)
- `entities/` – player, enemy, bullet, pickup
- `systems/` – movement, shooting, spawning, collision, events, cleanup
- `input.rs` – abstracts keyboard/mouse or touch input
- `event.rs` – in-game events for decoupled communication
- `ui/` – HUD and menu rendering
- `config.rs` – all tunable constants

This makes it easy to add new weapons, enemy types, or game states without rewriting existing code.
