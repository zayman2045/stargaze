//! Contains the systems that handle the layout of the HUD.

use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::hud::components::HUD;

/// Spawns the HUD.
pub fn spawn_hud(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    // Spawn the background for the HUD
    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::new(1.0, 1.0, 1.0),
                },
                texture: asset_server.load("sprites/space_background.png"),
                ..default()
            },
            HUD,
        ));
}

/// Despawns the HUD.
pub fn despawn_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    if let Ok(hud_entity) = hud_query.get_single() {
        commands.entity(hud_entity).despawn_recursive();
    }
}
