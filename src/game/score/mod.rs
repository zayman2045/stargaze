use bevy::prelude::*;

pub mod systems;
pub mod resources;

use systems::*;
use resources::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        // Resources
        .init_resource::<HighScore>()
        // On Enter Game State
        .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_system(update_score.run_if(in_state(AppState::Game)))
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        // On Exit Game State
        .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
        ;
    }
}