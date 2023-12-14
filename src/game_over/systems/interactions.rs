use bevy::ui::Interaction;
use bevy::{app::AppExit, prelude::*};

use crate::game_over::components::QuitButton;
use crate::{
    game_over::components::PlayAgainButton,
    styles::{CLICKED_BUTTON_COLOR, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR},
    AppState,
};

// Hover and click logic for the game over menu play again button
pub fn interact_with_play_again_button(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<PlayAgainButton>, Changed<Interaction>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                next_app_state.set(AppState::MainMenu)
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (With<QuitButton>, Changed<Interaction>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = CLICKED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
