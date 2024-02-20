//! Contains the systems for the player sprite.

use super::components::{DespawnSound, Player, StarSound};
use crate::events::GameOver;
use crate::game::asteroids::components::Asteroid;
use crate::game::asteroids::systems::ASTEROID_SIZE;
use crate::game::score::resources::Score;
use crate::game::stars::components::Star;
use crate::game::stars::systems::STAR_SIZE;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// The size of the player sprite
pub const PLAYER_SIZE: f32 = 100.0;
// The speed of the player sprite
pub const PLAYER_SPEED: f32 = 500.0;

/// Spawns the player sprite in the middle of the screen.
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Create a new entity with the SpriteBundle and Player components
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/playerShip1_blue.png"),
            ..default()
        },
        Player {},
    ));
}

/// Despawns the player.
pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

/// Moves the player sprite based on keyboard input.
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }

        // Normalize the direction vector so that diagonal movement isn't faster
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

/// Confines the player sprite to the window.
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let player_radius = PLAYER_SIZE / 2.0;
        let x_min = player_radius;
        let x_max = window.width() - player_radius;
        let y_min = player_radius;
        let y_max = window.height() - player_radius;

        let mut translation = player_transform.translation;

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

        player_transform.translation = translation;
    }
}

/// Despawns the player and sends a GameOver event if the player collides with an asteroid.
pub fn asteroid_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        // Check all asteroids for collisions
        for asteroid_transform in asteroid_query.iter() {
            let distance = player_transform
                .translation
                .distance(asteroid_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let asteroid_radius = ASTEROID_SIZE / 2.0;
            if distance < player_radius + asteroid_radius {

                // Play despawn sound
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/explosionCrunch_000.ogg"),
                        settings: PlaybackSettings::ONCE,
                    },
                    DespawnSound,
                ));

                // Despawn the player and send a GameOver event
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

/// Despawns the star and increment the score if the player collides with it.
pub fn player_collect_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // Check all stars for collisions
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                score.value += 1;

                // Play star sound
                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/confirmation_001.ogg"),
                        settings: PlaybackSettings::ONCE,
                    },
                    StarSound,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}
