use bevy::prelude::*;

use crate::{game::score::resources::HighScore, game_over::components::*, styles::*};

// Spawn the game over menu
pub fn spawn_game_over_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScore>,
) {
    // Get the final score
    let final_score = high_scores
        .scores
        .last()
        .unwrap_or(&("Player 1".to_string(), 0))
        .1;

    // Build and spawn the game over menu
    commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: MENU_STYLE,
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Game Over",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // === Final Score ===
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Final Score: {}", final_score),
                        get_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // ==== Play Again Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayAgainButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play Again",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // ==== Quit Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}
