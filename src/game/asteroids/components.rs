//! Contains the components related to the asteroids in the game.

use bevy::prelude::*;

/// Represents an asteroid in the game.
#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
}

/// Represents the sound of an asteroid. Used to play a sound when an asteroid is interacted with.
#[derive(Component)]
pub struct AsteroidSound;