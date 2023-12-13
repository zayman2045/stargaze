use crate::AppState;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

mod components;
mod styles;
mod systems;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter MainMenu State
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // OnExit MainMenu State
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // OnUpdate MainMenu State
            .add_systems(
                (interact_with_play_button, interact_with_quit_button)
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}
