//! Core functionality for the Stargaze game.
//!
//! This module contains the primary game logic and sets up the game environment,
//! including the initialization of various game states, systems, and plugins.
//!
//! The game is structured into several modules:
//! - `events`: Handles the game's events.
//! - `game`: Contains the main game logic.
//! - `game_over`: Handles the game over state.
//! - `main_menu`: Handles the main menu state.
//! - `states`: Defines the game's core states.
//! - `styles`: Contains the game's styles.
//! - `systems`: Contains the game's core systems.

mod events;
mod game;
mod game_over;
mod main_menu;
mod states;
mod styles;
mod systems;

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
