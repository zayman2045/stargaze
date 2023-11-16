use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<AsteroidSpawnTimer>()
            // On Enter Game State
            .add_system(spawn_asteroids.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (
                    asteroid_movement,
                    update_asteroid_direction,
                    confine_asteroid_movement,
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
