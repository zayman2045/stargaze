//! Defines the game states used in the Stargaze game.

use bevy::ecs::schedule::States;

/// The `AppState` enum represents the possible states of the Stargaze game.
///
#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    /// The default state, representing the main menu of the game.
    #[default]
    MainMenu,
    /// The state representing the gameplay.
    Game,
    /// The state representing the end of the game.
    GameOver,
}