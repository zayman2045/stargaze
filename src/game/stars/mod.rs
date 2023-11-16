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
            // Resources
            .init_resource::<StarSpawnTimer>()
            // Startup Systems
            .add_startup_system(spawn_stars)
            // Systems
            .add_system(tick_star_spawn_timer)
            .add_system(spawn_stars_over_time);
    }
}