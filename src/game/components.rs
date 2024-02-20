//! Contains the components related to the game state.

use bevy::ecs::component::Component;

/// The heads up display in the running simulation.
#[derive(Component)]
pub struct HUD;

/// The pause button in the HUD.
#[derive(Component)]
pub struct PauseButton;