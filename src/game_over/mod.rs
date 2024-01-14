//! This module contains the game over logic for the Stargaze game.
//!
//! The `GameOverPlugin` struct is the entry point for the game over logic.

use crate::states::AppState;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

pub mod components;
pub mod systems;

/// The primary plugin for the game over state in the Stargaze game.
///
/// This plugin adds the necessary systems for interacting with the game over menu to the Bevy app.
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