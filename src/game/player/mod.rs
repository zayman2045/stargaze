//! Contains the logic for the player in the game.
//! 
//! The `PlayerPlugin` struct is the entry point for the player logic.

pub mod components;
pub mod systems;

use super::SimulationState;
use crate::states::AppState;
use bevy::prelude::*;
use systems::*;

/// Represents a set of systems that handle the movement of the player.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

/// Represents a set of systems that confine the player within the game area.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

/// The primary plugin for the player in the game.
///
/// This plugin adds the necessary systems for the player to be spawned, despawned, and moved.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, MovementSystemSet.before(ConfinementSystemSet))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(OnExit(AppState::Game), despawn_player)
            // In-game Systems
            .add_systems(
                Update,
                (
                    player_movement.in_set(MovementSystemSet),
                    confine_player_movement.in_set(ConfinementSystemSet),
                )
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            )
            .add_systems(
                Update,
                (asteroid_hit_player, player_collect_star)
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            );
    }
}