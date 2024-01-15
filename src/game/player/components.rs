//! Contains the components related to the player in the game.

use bevy::prelude::*;

/// Represents the player in the game.
#[derive(Component)]
pub struct Player {}

/// Represents the sound played when the player is despawned.
#[derive(Component)]
pub struct DespawnSound;

/// Represents the sound played when the player collects a star.
#[derive(Component)]
pub struct StarSound;