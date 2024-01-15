//! Contains the systems related to the score in the game.

use bevy::prelude::*;
use super::resources::*;
use crate::events::GameOver;

/// Inserts the Score Resource into the world.
pub fn insert_score(
    mut commands: Commands
)  {
    commands.insert_resource(Score::default())
}

/// Removes the Score Resource from the world.
pub fn remove_score(
    mut commands: Commands
)  {
    commands.remove_resource::<Score>();
}

/// Adds the score to the list of high scores.
pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for game_over in game_over_event_reader.iter() {
        high_score
            .scores
            .push(("Player".to_string(), game_over.score));
    }
}