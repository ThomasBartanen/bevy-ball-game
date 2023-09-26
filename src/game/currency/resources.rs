use bevy::prelude::*;

#[derive(Resource)]
pub struct HeldCurrency {
    pub amount: f32
}

impl Default for HeldCurrency {
    fn default() -> HeldCurrency {
        HeldCurrency { amount: 100.0 }
    }
}