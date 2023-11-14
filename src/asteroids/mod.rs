use bevy::prelude::*;

pub mod components;
pub mod systems;
pub mod resources;

use systems::*;
use resources::*;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AsteroidSpawnTimer>()
            .add_startup_system(spawn_asteroids)
            .add_system(asteroid_movement)
            .add_system(update_asteroid_direction)
            .add_system(confine_asteroid_movement)
            .add_system(tick_asteroid_spawn_timer)
            .add_system(spawn_asteroids_over_time); 
            //.add_system(asteroid_hit_asteroid)
    }
}