//! Contains the logic for the HUD.
//!
//! The `HUDPlugin` struct is the entry point for the HUD logic.

pub mod components;
pub mod systems;

use bevy::prelude::*;

use systems::layout::{despawn_hud, spawn_hud};

use super::states::SimulationState;

/// The primary plugin for the HUD in the game.
///
/// This plugin adds the necessary systems for the HUD to be spawned, despawned, and interacted with.
pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SimulationState::Running), spawn_hud)
            .add_systems(OnEnter(SimulationState::Paused), despawn_hud);
    }
}
