//! Contains the systems that handle the interactions of the main menu buttons.

use bevy::{app::AppExit, prelude::*};

use crate::{
    main_menu::components::*,
    styles::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    AppState,
};

/// Changes the app state and ui style when the mouse interacts with the play button.
pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

/// Changes the app state and ui style when the mouse interacts with the quit button.
pub fn interact_with_quit_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
