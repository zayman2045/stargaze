mod events;
mod game;
mod game_over;
mod main_menu;
mod styles;
mod systems;
mod states;

use bevy::prelude::*;
use game::GamePlugin;
use game_over::GameOverPlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin, GameOverPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                enter_game_state,
                enter_main_menu_state,
                exit_game,
                handle_game_over,
            ),
        )
        .run()
}
