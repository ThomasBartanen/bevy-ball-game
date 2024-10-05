use bevy::prelude::*;

use crate::AppState;

pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .init_resource::<EnemyCount>()
            .add_systems(OnEnter(AppState::Game), (spawn_enemies_at_start,))
            .add_systems(
                Update,
                (
                    enemy_movement,
                    spin_enemies,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    tick_warning_timer,
                    spawn_warning_point,
                    spawn_enemies_over_time,
                    kill_enemies,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
