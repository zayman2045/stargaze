mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}

