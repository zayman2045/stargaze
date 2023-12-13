pub mod asteroids;
pub mod player;
pub mod score;
pub mod stars;
mod systems;

use crate::{events::GameOver, AppState};
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use stars::StarsPlugin;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            // Game Events
            .add_event::<GameOver>()
            // OnEnter Game State
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // Game Plugins
            .add_plugin(PlayerPlugin)
            .add_plugin(AsteroidsPlugin)
            .add_plugin(StarsPlugin)
            .add_plugin(ScorePlugin)
            // In-game Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // OnExit Game State
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

// States that the game can exist in
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
