pub mod player;
pub mod asteroids;
pub mod stars;
pub mod score;
mod systems;

use bevy::prelude::*;
use player::PlayerPlugin;
use asteroids::AsteroidsPlugin;
use stars::StarsPlugin;
use score::ScorePlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        // Events
        .add_event::<GameOver>()
        // Plugins
        .add_plugin(PlayerPlugin)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(StarsPlugin)
        .add_plugin(ScorePlugin)
        // Systems
        .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused
}