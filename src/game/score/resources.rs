//! Contains the resources related to the score in the game.

use bevy::prelude::*;

/// Represents the current score in the game.
#[derive(Resource)]
pub struct Score {
    /// The current score value.
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

/// Represents the high scores in the game.
#[derive(Resource, Debug)]
pub struct HighScore {
    /// A vector of tuples, each containing a player's name and their high score.
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScore {
    fn default() -> HighScore {
        HighScore { scores: Vec::new() }
    }
}