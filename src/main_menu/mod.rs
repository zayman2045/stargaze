//! Contains the main menu logic for the Stargaze game.
//!
//! The `MainMenuPlugin` struct is the entry point for the main menu logic.

use crate::states::AppState;
use bevy::prelude::*;
use systems::interactions::*;
use systems::layout::*;

pub mod components;
pub mod systems;

/// The primary plugin for the main menu state in the Stargaze game.
///
/// This plugin adds the necessary systems for interacting with the main menu to the Bevy app.
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
            .add_systems(
                Update,
                interact_with_play_button
                    .run_if(in_state(AppState::MainMenu)),
            );
    }
}