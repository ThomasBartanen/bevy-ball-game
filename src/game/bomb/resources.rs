use bevy::prelude::*;

#[derive(Resource)]
pub struct PlacedBombCount {
    pub count: usize
}

impl Default for PlacedBombCount {
    fn default() -> PlacedBombCount {
        PlacedBombCount { count: 0 }
    }
}

#[derive(Resource, Deref)]
pub struct HeldBombCount {
    pub count: usize
}

impl Default for HeldBombCount {
    fn default() -> HeldBombCount {
        HeldBombCount { count: 3 }
    }
}

#[derive(Resource, Deref)]
pub struct BombCooldownTimer {
    pub timer: Timer
}

impl Default for BombCooldownTimer {
    fn default() -> BombCooldownTimer {
        BombCooldownTimer {
            timer: Timer::default()
        }
    }
}