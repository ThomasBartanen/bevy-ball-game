use bevy::prelude::*;

use crate::constants::PAUSE_GAME_KEY;

use super::SimulationState;

pub fn pause_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>
) {
    next_sim_state.set(SimulationState::Paused);
}

pub fn resume_simulation(
    mut next_sim_state: ResMut<NextState<SimulationState>>
) {
    next_sim_state.set(SimulationState::Running);
}

pub fn toggle_simulation(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_app_state: ResMut<NextState<SimulationState>>
) {
    if keyboard_input.just_pressed(PAUSE_GAME_KEY) {
        if simulation_state.get() == &SimulationState::Running {
            next_app_state.set(SimulationState::Paused);
            println!("Simulation Paused.");
        }
        if simulation_state.get() == &SimulationState::Paused {
            next_app_state.set(SimulationState::Running);
            println!("simulation Running.");
        }
    }
}