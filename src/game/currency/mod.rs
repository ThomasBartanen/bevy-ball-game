use super::SimulationState;
use crate::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HeldCurrency>()
            .add_systems(OnEnter(AppState::Game), insert_currency)
            .add_systems(
                Update,
                (purchase_bomb, get_currency_on_kill)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), remove_currency);
    }
}
