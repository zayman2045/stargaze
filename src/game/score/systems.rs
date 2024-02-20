//! Contains the systems related to the score in the game.

use super::resources::*;
use bevy::prelude::*;

/// Inserts the Score Resource into the world.
pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default())
}

/// Removes the Score Resource from the world.
pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}
