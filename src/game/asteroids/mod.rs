use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct DirectionSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<AsteroidSpawnTimer>()
            // Configure System Sets
            .configure_set(MovementSystemSet.before(DirectionSystemSet))
            .configure_set(DirectionSystemSet.before(ConfinementSystemSet))
            // On Enter Game State
            .add_system(spawn_asteroids.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    asteroid_movement.in_set(MovementSystemSet),
                    update_asteroid_direction.in_set(DirectionSystemSet),
                    confine_asteroid_movement.in_set(ConfinementSystemSet),
                    tick_asteroid_spawn_timer,
                    spawn_asteroids_over_time,
                )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running)),
            )
            // On Exit Game State
            .add_system(despawn_asteroids.in_schedule(OnExit(AppState::Game)));
    }
}
