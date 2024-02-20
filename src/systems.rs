//! Contains the systems used in the game.

use crate::events::GameOver;
use crate::states::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Spawns the camera in the middle of the screen.
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

/// Exits the game when the "Escape" key is pressed.
pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

/// Prints the final score when the game ends.
pub fn handle_game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for game_over in game_over_event_reader.iter() {
        println!("Game Over! Final Score: {}", game_over.score);
        next_app_state.set(AppState::GameOver)
    }
}