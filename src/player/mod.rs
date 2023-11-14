use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(confine_player_movement)
            .add_system(asteroid_hit_player)
            .add_system(player_collect_star);

    }
}