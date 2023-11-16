use bevy::prelude::*;
use super::resources::*;
use crate::events::GameOver;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

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

pub fn high_scores_updated(high_scores: Res<HighScore>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}