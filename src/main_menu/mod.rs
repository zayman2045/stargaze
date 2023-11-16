use bevy::prelude::*;

use crate::AppState;

use systems::layout::*;

mod systems;
mod components;
mod styles;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        // On Enter State Systems
        .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
        // On Exit State Systems
        .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)))
        ;
    }
}
