//! Contains the resources related to the stars in the game.

use bevy::prelude::*;

pub const STAR_SPAWN_TIME: f32 = 1.0;

/// Represents a timer for spawning stars in the game.
#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}