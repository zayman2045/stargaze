//! Contains the systems related to the asteroids in the game.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use super::components::{Asteroid, AsteroidSound};
use super::resources::AsteroidSpawnTimer;

pub const NUMBER_OF_ASTEROIDS: usize = 4;
pub const ASTEROID_SIZE: f32 = 100.0;
pub const ASTEROID_SPEED: f32 = 200.0;

/// Spawns the asteroids in random locations.
pub fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let (window_width, window_height) = (window.width(), window.height());
    let mut initial_spawn_locations = [(0.1, 0.1), (0.1, 0.9), (0.9, 0.1), (0.9, 0.9)].iter().map(|(x, y)| (x * window_width, y * window_height));

    for _ in 0..NUMBER_OF_ASTEROIDS {
        let (x, y) = initial_spawn_locations.next().expect("There should be one unique spawn location per asteroid");

        // Create a new entity with the SpriteBundle and Asteroid components
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/meteorBrown_big1.png"),
                ..default()
            },
            Asteroid {
                // Generate a random direction for the asteroid
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

/// Despawns all of the asteroids.
pub fn despawn_asteroids(mut commands: Commands, asteroid_query: Query<Entity, With<Asteroid>>) {
    for asteroid_entity in asteroid_query.iter() {
        commands.entity(asteroid_entity).despawn();
    }
}

/// Moves the asteroids based on their direction.
pub fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut asteroid_transform, asteroid) in asteroid_query.iter_mut() {
        let direction = Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0);
        asteroid_transform.translation += direction * ASTEROID_SPEED * time.delta_seconds();
    }
}

/// Reverses the direction of an asteroid if it collides with the window edge.
pub fn update_asteroid_direction(
    mut commands: Commands,
    mut asteroid_query: Query<(&Transform, &mut Asteroid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let asteroid_radius = ASTEROID_SIZE / 2.0;
    let x_min = asteroid_radius;
    let x_max = window.width() - asteroid_radius;
    let y_min = asteroid_radius;
    let y_max = window.height() - asteroid_radius;

    for (asteroid_transform, mut asteroid) in asteroid_query.iter_mut() {
        let mut direction_changed = false;

        let translation = asteroid_transform.translation;

        if translation.x < x_min || translation.x > x_max {
            asteroid.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            asteroid.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play a sound when the asteroid changes direction
        if direction_changed {
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("audio/impactSoft_heavy_000.ogg"),
                    settings: PlaybackSettings::ONCE,
                },
                AsteroidSound,
            ));
        }
    }
}

/// Confines the asteroids to the window.
pub fn confine_asteroid_movement(
    mut asteroid_query: Query<&mut Transform, With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let asteroid_radius = ASTEROID_SIZE / 2.0;
    let x_min = asteroid_radius;
    let x_max = window.width() - asteroid_radius;
    let y_min = asteroid_radius;
    let y_max = window.height() - asteroid_radius;

    for mut asteroid_transform in asteroid_query.iter_mut() {
        let mut translation = asteroid_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        asteroid_transform.translation = translation;
    }
}

/// Reverses the direction of an asteroids if it collides with another asteroid.
fn _asteroid_hit_asteroid(mut asteroid_query: Query<(&Transform, &mut Asteroid)>) {
    let mut combinations = asteroid_query.iter_combinations_mut();

    while let Some(
        [(asteroid_transform_1, mut asteroid_1), (asteroid_transform_2, mut asteroid_2)],
    ) = combinations.fetch_next()
    {
        let distance = asteroid_transform_1
            .translation
            .distance(asteroid_transform_2.translation);
        let asteroid_radius = ASTEROID_SIZE / 2.0;
        if distance < asteroid_radius * 2.0 {
            asteroid_1.direction.x *= -1.0;
            asteroid_1.direction.y *= -1.0;
            asteroid_2.direction.x *= -1.0;
            asteroid_2.direction.y *= -1.0;
        }
    }
}

/// Ticks the timer controlling the spawning of asteroids.
pub fn tick_asteroid_spawn_timer(
    mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>,
    time: Res<Time>,
) {
    asteroid_spawn_timer.timer.tick(time.delta());
}

/// Spawns an asteroid if the timer has finished.
pub fn spawn_asteroids_over_time(
    mut commands: Commands,
    asteroid_spawn_timer: Res<AsteroidSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if asteroid_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        // Create a new entity with the SpriteBundle and Asteroid components
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/meteorBrown_big1.png"),
                ..default()
            },
            Asteroid {
                // Generate a random direction for the asteroid
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}