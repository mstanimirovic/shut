# User Interface (`ui/hud.rs`)

The HUD is drawn by `draw_hud(game: &Game)`.

It creates a semi‑transparent panel at the bottom of the screen and shows:

- **Health**: green text, `HP: current/max`
- **Ammo**: yellow text, either `magazine / max (reserve reserve)` or `Reloading...` with countdown
- **Enemies count**: red text
- **Score**: white text

Coordinates are hardcoded for a 800×600 window; consider using a more flexible layout system for variable resolutions.
