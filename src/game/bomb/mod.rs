use bevy::prelude::*;

use crate::AppState;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlacedBombCount>()
        .init_resource::<HeldBombCount>()
        .init_resource::<BombCooldownTimer>()

        .add_systems(
            OnEnter(AppState::Game), 
            insert_bombs)

        .add_systems(
            Update, 
            (
                tick_bomb_timers,
                detonate_bomb,
                tick_explosion_timers,
                tick_cooldown_timer,
                enemy_hit_explosion,
                remove_explosion
            )
            .run_if(in_state(AppState::Game))
            .run_if(in_state(SimulationState::Running))
        )

        .add_systems(
            OnExit(AppState::Game), 
            (
                despawn_bombs,
                remove_bombs
        ));
    }
}