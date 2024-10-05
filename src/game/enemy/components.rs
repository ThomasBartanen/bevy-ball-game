use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct WarningCircle {
    pub spawn_time: f32,
}
