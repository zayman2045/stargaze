//! Contains the gameplay logic for the Stargaze game.
//!
//! The `GamePlugin` struct is the entry point for the gameplay logic.

pub mod asteroids;
pub mod hud;
pub mod player;
pub mod score;
pub mod stars;
pub mod states;
pub mod systems;

use crate::{events::GameOver, AppState};
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use hud::HUDPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use stars::StarsPlugin;
use states::SimulationState;
use systems::*;

/// The primary plugin for gameplay within the Stargaze game.
///
/// This plugin adds the necessary states, events, plugins, and systems to the Bevy app.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((
                PlayerPlugin,
                AsteroidsPlugin,
                StarsPlugin,
                ScorePlugin,
                HUDPlugin,
            ))
            .add_systems(OnEnter(AppState::Game), run_simulation)
            .add_systems(OnExit(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}
