use bevy::prelude::*;

mod player;
pub mod bomb;
pub mod currency;
pub mod enemy;
pub mod score;
pub mod star;
pub mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use bomb::BombPlugin;
use systems::*;

use crate::AppState;
use crate::events::*;


pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App) {
        app    
        .add_state::<SimulationState>()

        .add_event::<GameOver>()

        .add_systems(
            OnEnter(AppState::Game), 
            pause_simulation)

        .add_plugins((
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
            BombPlugin
        ))

        .add_systems(
            Update,
            toggle_simulation
        .run_if(in_state(AppState::Game))
        )

        .add_systems(
            OnExit(AppState::Game), 
            resume_simulation
        );
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused
}