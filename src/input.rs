use macroquad::prelude::*;

/// Unified input for both keyboard/mouse and touch.
/// This abstracts away the actual physical device.
pub struct InputState {
    pub move_dir: Vec2,
    pub aim_dir: Vec2,
    pub shoot: bool,
    pub reload: bool,
    pub confirm: bool,
    pub pause: bool,
}

impl InputState {
    /// Build input from keyboard and mouse.
    pub fn from_keyboard_and_mouse(player_pos: Vec2) -> Self {
        let mut move_dir = Vec2::ZERO;
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        let mouse_pos: Vec2 = mouse_position().into();
        let aim_dir = if mouse_pos.distance(player_pos) > 0.001 {
            (mouse_pos - player_pos).normalize()
        } else {
            Vec2::X // default right
        };

        Self {
            move_dir,
            aim_dir,
            shoot: is_mouse_button_down(MouseButton::Left),
            reload: is_key_pressed(KeyCode::R),
            confirm: is_key_pressed(KeyCode::Enter),
            pause: is_key_pressed(KeyCode::Escape),
        }
    }

    /// Build input from touch controls (for mobile).
    /// Here we just provide a stub; a real implementation would use
    /// a virtual joystick crate.
    pub fn from_touch(_player_pos: Vec2) -> Self {
        // Placeholder – return empty input
        Self {
            move_dir: Vec2::ZERO,
            aim_dir: Vec2::X,
            shoot: false,
            reload: false,
            confirm: false,
            pause: false,
        }
    }
}
