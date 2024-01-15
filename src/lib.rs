//! Core functionality for the Stargaze game.
//!
//! Contains the primary game logic and sets up the game environment,
//! including the initialization of various game states, systems, and plugins.

pub mod events;
pub mod game;
pub mod game_over;
pub mod main_menu;
pub mod states;
pub mod styles;
pub mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use game_over::GameOverPlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::*;

/// Runs the Stargaze game.
///
/// This function sets up the game environment using the Bevy game engine.
/// It initializes the game state, registers game plugins and systems, and
/// starts the game loop.
///
/// # Example
///
/// ```
/// stargaze::run_game();
/// ```
pub fn run_game() {
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
