//! Contains the components related to the game over menu in the game.

use bevy::ecs::component::Component;

/// Represents the game over menu in the game.
#[derive(Component)]
pub struct GameOverMenu;

/// Represents the play again button in the game over menu.
#[derive(Component)]
pub struct PlayAgainButton;