use bevy::prelude::*;

#[derive(Resource)]
pub struct BombCount {
    pub count: usize
}

impl Default for BombCount {
    fn default() -> BombCount {
        BombCount { count: 0 }
    }
}