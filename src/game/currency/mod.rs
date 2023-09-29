use bevy::prelude::*;
use crate::AppState;
use super::SimulationState;
use resources::*;
use systems::*;

pub mod components;
pub mod resources;
pub mod systems;


pub struct CurrencyPlugin;

impl Plugin for CurrencyPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<HeldCurrency>()

        .add_systems(
            OnEnter(AppState::Game), 
            insert_currency)

        .add_systems(
            Update, 
            (
                purchase_bomb,
                update_currency
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )
        .add_systems(
            OnExit(AppState::Game),
            remove_currency
        );
    }
}