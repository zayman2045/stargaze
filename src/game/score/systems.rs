use bevy::prelude::*;
use super::resources::*;
use crate::events::GameOver;

// Insert the Score Resource
pub fn insert_score(
    mut commands: Commands
)  {
    commands.insert_resource(Score::default())
}

// Remove the Score Resource
pub fn remove_score(
    mut commands: Commands
)  {
    commands.remove_resource::<Score>();
}

// Print the score as it changes
pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

// Add high score upon GameOver event
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

// Print the high scores if they change
pub fn high_scores_updated(high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}