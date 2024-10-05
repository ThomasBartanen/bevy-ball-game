use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlacedBombCount {
    pub count: usize,
}

#[derive(Resource, Deref)]
pub struct HeldBombCount {
    pub count: usize,
}

impl Default for HeldBombCount {
    fn default() -> HeldBombCount {
        HeldBombCount { count: 3 }
    }
}

#[derive(Resource, Deref, Default)]
pub struct BombCooldownTimer {
    pub timer: Timer,
}
