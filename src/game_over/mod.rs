use crate::states::AppState;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

mod components;
mod styles;
mod systems;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_menu)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over_menu)
            .add_systems(
                Update,
                (interact_with_play_again_button, interact_with_quit_button)
                    .run_if(in_state(AppState::GameOver)),
            );
    }
}
