use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 100.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ASTEROIDS: usize = 4;
pub const ASTEROID_SIZE: f32 = 100.0;
pub const ASTEROID_SPEED: f32 = 200.0;
pub const ASTEROID_SPAWN_TIME: f32 = 5.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<AsteroidSpawnTimer>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_asteroids)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .add_system(asteroid_movement)
        .add_system(update_asteroid_direction)
        .add_system(confine_asteroid_movement)
        .add_system(asteroid_hit_player)
        //.add_system(asteroid_hit_asteroid)
        .add_system(player_collect_star)
        .add_system(update_score)
        .add_system(tick_star_spwan_timer)
        .add_system(spawn_stars_over_time)
        .add_system(tick_asteroid_spwan_timer)
        .add_system(spawn_asteroids_over_time)
        .add_system(exit_game)
        .run()
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct AsteroidSpawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidSpawnTimer {
    fn default() -> AsteroidSpawnTimer {
        AsteroidSpawnTimer {
            timer: Timer::from_seconds(ASTEROID_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

// Spawn the player sprite in the middle of the screen
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

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

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

        // Normalize the direction vector so that diagonal movement isn't faster
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
    player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
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

// Reverse the direction of the asteroids if they collide with each other
pub fn asteroid_hit_asteroid(mut asteroid_query: Query<(&Transform, &mut Asteroid)>) {
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

pub fn player_collect_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let star_radius = STAR_SIZE / 2.0;
            if distance < player_radius + star_radius {
                println!("Player Collected Star!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/confirmation_001.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spwan_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

// Spawn a star if the timer has finished
pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

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

pub fn tick_asteroid_spwan_timer(mut asteroid_spawn_timer: ResMut<AsteroidSpawnTimer>, time: Res<Time>) {
    asteroid_spawn_timer.timer.tick(time.delta());
}

// Spawn an asteroid if the timer has finished
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

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}