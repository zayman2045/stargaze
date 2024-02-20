//! Contains the systems that handle the layout of the main menu.

use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::styles::*;

/// Spawns the main menu.
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK),
                style: MENU_STYLE,
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // ==== Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Image 1
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/playerShip1_blue.png").into(),
                        ..default()
                    });
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "STARGAZE",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Image 2
                    parent.spawn(ImageBundle {
                        style: IMAGE_STYLE,
                        image: asset_server.load("sprites/playerShip1_blue.png").into(),
                        ..default()
                    });
                });
            // ==== Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
            // ==== Directions Title ===
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Directions:",
                        get_directions_title_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            // ==== Directions Body ===
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "- Use the WASD or arrow keys to move your ship",
                            get_directions_body_text_style(&asset_server),
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection::new(
                            "- Press the spacebar to play and pause the game",
                            get_directions_body_text_style(&asset_server),
                        )
                    ],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();
}

/// Despawns the main menu.
pub fn despawn_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(menu_entity) = menu_query.get_single() {
        commands.entity(menu_entity).despawn_recursive();
    }
}
