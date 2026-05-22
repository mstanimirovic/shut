# Main Loop (`main.rs`)

The entry point is the `async fn main` decorated with `#[macroquad::main("shut")]`.

**Flow per frame**:

1. `clear_background(LIGHTGRAY)`
2. Build an `InputState` from keyboard/mouse.
3. Call `update_state(...)` to possibly switch between Menu/Playing/Dead.
4. If in `Playing` state, run all systems in this order:
   - `movement::update_movement`
   - `shooting::update_shooting`
   - `spawner::update_spawner`
   - `collision::check_collisions`
   - `events::process_events`
   - `cleanup::cleanup_entities`
5. Render all entities (bullets, enemies, pickups, player).
6. Draw the HUD if in `Playing` or `Dead`.
7. Draw state‑specific UI (menu, game over).
8. `next_frame().await`

**Why `async`?** Macroquad uses it for cross‑platform support (especially WASM), where `next_frame().await` yields control to the browser.
