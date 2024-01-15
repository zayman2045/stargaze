//! Contains the logic for managing the stars in the game.
//! 
//! The `StarsPlugin` struct is the entry point for the stars logic.

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::states::AppState;
use bevy::prelude::*;

/// The primary plugin for the stars in the game.
///
/// This plugin adds the necessary systems for the stars to be spawned, despawned, and updated.
pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(OnExit(AppState::Game), despawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            );
    }
}