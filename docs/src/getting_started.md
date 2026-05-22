# Getting Started

## Prerequisites

- Rust toolchain (stable)
- Cargo

## Building & Running

```bash
# Clone the repository
git clone <your-repo-url>
cd shut

# Run natively
cargo run --release

# Build for web (WASM)
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

Place optional assets (textures, sounds) in the `assets/` folder. The game works without them – everything falls back to colored shapes.

## Controls

| Action          | Keyboard/Mouse        |
|-----------------|-----------------------|
| Move            | `WASD` / Arrow keys   |
| Aim             | Mouse position        |
| Shoot           | Left mouse button     |
| Reload          | `R`                   |
| Confirm (menu)  | `Enter`               |
