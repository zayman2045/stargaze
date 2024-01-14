pub mod resources;
pub mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(OnExit(AppState::Game), remove_score)
            .add_systems(
                Update,
                (
                    update_high_scores,
                    high_scores_updated,
                    update_score.run_if(in_state(AppState::Game)),
                ),
            );
    }
}
