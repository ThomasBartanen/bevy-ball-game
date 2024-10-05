use bevy::prelude::*;

#[derive(Component)]
pub struct Bomb {
    pub det_time: f32,
}

#[derive(Component)]
pub struct Explosion {
    pub visible_time: f32,
}

#[derive(Component)]
pub struct BloodPool {}

#[derive(Component)]
pub struct ScorchMark {}
