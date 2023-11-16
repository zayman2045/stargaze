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
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // My Plugins
        .add_plugin(GamePlugin)
        .add_plugin(MainMenuPlugin)
        // Startup Systems
        .add_startup_system(spawn_camera)
        // Systems
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run()
}

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}