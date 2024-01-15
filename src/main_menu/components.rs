//! Contains the components related to the main menu in the game.

use bevy::prelude::Component;

/// Represents the main menu in the game.
#[derive(Component)]
pub struct MainMenu;

/// Represents the play button in the main menu.
#[derive(Component)]
pub struct PlayButton;

/// Represents the quit button in the main menu.
#[derive(Component)]
pub struct QuitButton;