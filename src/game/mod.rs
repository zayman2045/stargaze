pub mod player;
pub mod asteroids;
pub mod stars;
pub mod score;

use bevy::prelude::*;
use player::PlayerPlugin;
use asteroids::AsteroidsPlugin;
use stars::StarsPlugin;
use score::ScorePlugin;

use crate::events::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
        .add_plugin(PlayerPlugin)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(StarsPlugin)
        .add_plugin(ScorePlugin);
    }
}