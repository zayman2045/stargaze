//! This module contains the logic for the asteroids in the game.
//! 
//! The `AsteroidsPlugin` struct is the entry point for the asteroids logic.

pub mod components;
pub mod resources;
pub mod systems;

use super::SimulationState;
use crate::states::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

/// Represents a set of systems that handle the movement of asteroids.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

/// Represents a set of systems that handle the direction of asteroids.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct DirectionSystemSet;

/// Represents a set of systems that confine the asteroids within the game area.
#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

/// The primary plugin for the asteroids in the game.
///
/// This plugin adds the necessary systems for asteroids to be spawned, despawned, and moved.
pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>()
            .configure_set(Update, MovementSystemSet.before(DirectionSystemSet))
            .configure_set(Update, DirectionSystemSet.before(ConfinementSystemSet))
            .add_systems(OnEnter(AppState::Game), spawn_asteroids)
            .add_systems(OnExit(AppState::Game), despawn_asteroids)
            .add_systems(
                Update,
                (
                    asteroid_movement.in_set(MovementSystemSet),
                    update_asteroid_direction.in_set(DirectionSystemSet),
                    confine_asteroid_movement.in_set(ConfinementSystemSet),
                    tick_asteroid_spawn_timer,
                    spawn_asteroids_over_time,
                )
                    .run_if(in_state(AppState::Game).and_then(in_state(SimulationState::Running))),
            );
    }
}
