pub mod components;
pub mod systems;

use super::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Clone, Hash, PartialEq, Eq)]
pub struct ConfinementSystemSet;

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
