use bevy::prelude::*;

#[derive(Component)]
pub struct Asteroid {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct AsteroidSound;