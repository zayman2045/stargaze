use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use super::SimulationState;
use crate::AppState;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<StarSpawnTimer>()
            // OnEnter Game State
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            // In-game Systems
            .add_systems(
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            // OnExit Game State
            .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
