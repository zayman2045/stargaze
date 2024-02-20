//! Contains systems related to the stars in the game.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use super::components::Star;
use super::resources::StarSpawnTimer;

pub const INITIAL_NUMBER_OF_STARS: usize = 10;
pub const ADDITIONAL_STARS_PER_SPAWN: usize = 2;
pub const STAR_SIZE: f32 = 30.0;

/// Spawns the initial stars in the game.
pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();

    for _ in 0..INITIAL_NUMBER_OF_STARS {
        // Generate a random location for the star
        let random_x = rng.gen_range(0.05 * window.width()..0.95 * window.width());
        let random_y = rng.gen_range(0.05 * window.height()..0.95 * window.height());

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star_gold.png"),
                ..default()
            },
            Star {},
        ));
    }
}

/// Despawns all stars in the game.
pub fn despawn_stars(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

/// Ticks the star spawn timer.
pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

/// Spawns stars over time.
pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let mut rng = rand::thread_rng();

        for _ in 0..ADDITIONAL_STARS_PER_SPAWN {
            // Generate a random location for the star
            let random_x = rng.gen_range(0.05 * window.width()..0.95 * window.width());
            let random_y = rng.gen_range(0.05 * window.height()..0.95 * window.height());

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(random_x, random_y, 0.0),
                    texture: asset_server.load("sprites/star_gold.png"),
                    ..default()
                },
                Star {},
            ));
        }
    }
}
