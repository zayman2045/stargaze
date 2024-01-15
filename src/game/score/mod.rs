//! Contains the logic for managing the score in the game.
//!
//! The `ScorePlugin` struct is the entry point for the score logic.

pub mod resources;
pub mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

/// The primary plugin for the score in the game.
///
/// This plugin adds the necessary systems for the score to be inserted, removed, and updated.
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score)
            .add_systems(Update, update_high_scores);
    }
}
