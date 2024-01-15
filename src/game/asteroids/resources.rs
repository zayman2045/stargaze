//! Contains the resources related to the asteroids in the game.

use bevy::prelude::*;

/// The number of seconds between each asteroid spawn.
pub const ASTEROID_SPAWN_TIME: f32 = 5.0;

/// Represents a timer for spawning asteroids in the game.
#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> AsteroidSpawnTimer {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}