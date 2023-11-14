mod player;
mod asteroids;
mod stars;
mod score;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use score::resources::Score;
use stars::resources::StarSpawnTimer;
use asteroids::resources::AsteroidSpawnTimer;
use score::resources::HighScore;
use player::systems::*;
use asteroids::systems::*;
use stars::systems::*;
use score::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<AsteroidSpawnTimer>()
        .init_resource::<HighScore>()
        .add_event::<GameOver>()
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
        .add_system(handle_game_over)
        .add_system(update_high_scores)
        .add_system(high_scores_updated)
        .run()
}

pub struct GameOver {
    pub score: u32,
}


// Spawn the camera in the middle of the screen
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}



pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for game_over in game_over_event_reader.iter() {
        println!("Game Over! Final Score: {}", game_over.score);
    }
}
