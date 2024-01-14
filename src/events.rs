//! This module defines the events used in the Stargaze game.

use bevy::ecs::event::Event;

/// The `GameOver` event is triggered when the game ends.
///
/// This event carries the final score of the game.
#[derive(Event)]
pub struct GameOver {
    /// The final score achieved in the game.
    pub score: u32,
}