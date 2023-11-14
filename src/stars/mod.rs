use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use systems::*;
use resources::*;

pub struct StarsPlugin;

impl Plugin for StarsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}