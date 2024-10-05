use bevy::prelude::*;

pub mod bomb;
pub mod currency;
pub mod enemy;
mod hud;
mod player;
pub mod score;
pub mod sound;
pub mod star;
pub mod systems;

use bomb::BombPlugin;
use enemy::EnemyPlugin;
use hud::HUDPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use sound::*;
use star::StarPlugin;
use systems::*;

use crate::events::*;
use crate::AppState;

use self::currency::CurrencyPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_event::<EnemyKilled>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins((
                HUDPlugin,
                EnemyPlugin,
                CurrencyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                SoundPlugin,
                BombPlugin,
            ))
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
