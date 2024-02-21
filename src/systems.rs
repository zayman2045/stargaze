//! Contains the systems used in the game.

use crate::events::GameOver;
use crate::game::score::resources::HighScore;
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

/// Updates the high scores and sets the next app state to GameOver.
pub fn handle_game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for game_over in game_over_event_reader.iter() {
        high_score
            .scores
            .push(("Player 1".to_string(), game_over.score));
        
        next_app_state.set(AppState::GameOver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_game_on_escape_press() {
        // Setup app
        let mut app = App::new();

        // Add AppExit event
        app.add_event::<AppExit>();

        // Add exit_game system
        app.add_systems(Update, exit_game);

        // Setup test resource with Escape key just pressed
        let mut keyboard_input = Input::<KeyCode>::default();
        keyboard_input.press(KeyCode::Escape);
        app.insert_resource(keyboard_input);

        // Run systems
        app.update();

        // Get AppExit event reader
        let app_exit_events = app.world.resource::<Events<AppExit>>();
        let mut app_exit_reader = app_exit_events.get_reader();
        let app_exited = app_exit_reader.iter(app_exit_events).next().is_some();

        // Check the event has been sent
        assert!(
            app_exited,
            "AppExit event should be sent when Escape is pressed"
        );
    }

    #[test]
    fn handle_game_over_updates_scores_and_state() {
        // Setup app
        let mut app = App::new();

        // Add necessary resources and events
        app.insert_resource(HighScore::default())
            .add_state::<AppState>()
            .add_event::<GameOver>();

        // Add the handle_game_over system
        app.add_systems(Update, handle_game_over);

        // Send a GameOver event
        app.world
            .resource_mut::<Events<GameOver>>()
            .send(GameOver { score: 100 });

        // Run systems
        app.update();

        // Check HighScore was updated
        let high_scores = app.world.resource::<HighScore>();
        assert_eq!(high_scores.scores.len(), 1);
        assert_eq!(high_scores.scores[0], ("Player 1".to_string(), 100));

        // Check AppState is set to GameOver
        let next_state = app.world.resource::<NextState<AppState>>().0.unwrap();
        assert_eq!(next_state, AppState::GameOver);
    }
}
