use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 100.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ASTEROIDS: usize = 4;
pub const ASTEROID_SIZE: f32 = 100.0;
pub const ASTEROID_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_asteroids)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(asteroid_movement)
        .add_system(update_asteroid_direction)
        .add_system(confine_asteroid_movement)
        .add_system(asteroid_hit_player)
        .run()
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
}

// Spawn the player sprite in the middle of the screen
pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/playerShip1_blue.png"),
            ..default()
        },
        Player {},
    ));
}

// Spawn the camera in the middle of the screen
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

// Spawn the asteroids in random locations
pub fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ASTEROIDS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/meteorBrown_big1.png"),
                ..default()
            },
            Asteroid {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

// Move the player sprite based on keyboard input
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

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// Confine the player sprite to the screen
pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let player_radius = PLAYER_SIZE / 2.0; // 32.0
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

// Move the asteroids based on their direction
pub fn asteroid_movement(mut asteroid_query: Query<(&mut Transform, &Asteroid)>, time: Res<Time>) {
    for (mut asteroid_transform, asteroid) in asteroid_query.iter_mut() {
        let direction = Vec3::new(asteroid.direction.x, asteroid.direction.y, 0.0);
        asteroid_transform.translation += direction * ASTEROID_SPEED * time.delta_seconds();
    }
}

// Reverse the direction of the asteroid if it hits the edge of the screen
pub fn update_asteroid_direction(
    mut asteroid_query: Query<(&Transform, &mut Asteroid)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
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
            let sound_effect_1 = asset_server.load("audio/impactSoft_heavy_000.ogg");
            let sound_effect_2 = asset_server.load("audio/impactSoft_heavy_001.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            audio.play(sound_effect);
        }
    }
}

// Confine the asteroids to the screen
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

// Destroy the player if it collides with an asteroid
pub fn asteroid_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for asteroid_transform in asteroid_query.iter() {
            let distance = player_transform
                .translation
                .distance(asteroid_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let asteroid_radius = ASTEROID_SIZE / 2.0;
            if distance < player_radius + asteroid_radius {
                println!("Player Destroyed!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
            }
        }
    }
}
