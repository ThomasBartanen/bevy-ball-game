use crate::AppState;
use bevy::prelude::*;

use systems::{layout::*, updates::*};

use super::SimulationState;

//use self::systems::interactions::{interact_with_play_button, interact_with_quit_button};
//use systems::interactions::*;

mod components;
mod styles;
mod systems;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(
                Update,
                (
                    update_score,
                    update_currency,
                    update_kill_count,
                    update_bombs,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), despawn_hud);
    }
}
