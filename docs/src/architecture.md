# Architecture Overview

The code follows a **data‑oriented, modular design**. Key principles:

1. **`Game`** struct holds all runtime state (player, enemies, bullets, pickups, events).
2. **Systems** are plain functions that take `&mut Game` and `dt` (delta time). They modify the game state without owning it.
3. **Components** are plain data structures reused across entities (e.g. `Health` appears in both `Player` and `Enemy`).
4. **Events** decouple systems: collisions push `GameEvent` variants into a queue; the event processing system consumes them and updates the game accordingly.
5. **Input** is abstracted into `InputState`, so the game logic never calls `is_key_down()` directly – making it easy to add touch or AI controls.
6. **Game states** (`Menu`, `Playing`, `Dead`) control which systems run and what UI is shown.

This separation allows you to add a new enemy type or weapon without touching existing logic, and test systems in isolation.
