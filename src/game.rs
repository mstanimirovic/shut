use crate::entities::bullet::Bullet;
use crate::entities::enemy::Enemy;
use crate::entities::pickup::AmmoPickup;
use crate::entities::player::Player;
use crate::event::GameEvent;
use crate::resources::Resources;

/// Central game state – holds everything.
pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub pickups: Vec<AmmoPickup>,
    pub score: u32,
    pub spawn_timer: f32,
    pub events: Vec<GameEvent>,
    pub resources: Option<Resources>,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            player: Player::new(),
            enemies: Vec::new(),
            bullets: Vec::new(),
            pickups: Vec::new(),
            score: 0,
            spawn_timer: 0.0,
            events: Vec::new(),
            resources: Resources::new().await.ok(),
        }
    }

    /// Reset game to initial state (for new game / restart).
    pub async fn reset(&mut self) {
        *self = Game::new().await;
    }
}
