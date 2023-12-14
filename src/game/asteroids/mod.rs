pub mod components;
pub mod resources;
pub mod systems;

use super::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct DirectionSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AsteroidSpawnTimer>()
            .configure_set(Update, MovementSystemSet.before(DirectionSystemSet))
            .configure_set(Update, DirectionSystemSet.before(ConfinementSystemSet))
            .add_systems(OnEnter(AppState::Game), spawn_asteroids)
            .add_systems(OnExit(AppState::Game), despawn_asteroids)
            // In-game Systems
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
        // OnExit Game State
    }
}
