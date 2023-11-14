mod player;
mod asteroids;
mod stars;
mod score;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use player::PlayerPlugin;
use asteroids::AsteroidsPlugin;
use stars::StarsPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(AsteroidsPlugin)
        .add_plugin(StarsPlugin)
        .add_plugin(ScorePlugin)
        .add_event::<GameOver>()
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
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

// Exit the game when the escape key is pressed
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

// Print the final score when the game ends
pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for game_over in game_over_event_reader.iter() {
        println!("Game Over! Final Score: {}", game_over.score);
    }
}
