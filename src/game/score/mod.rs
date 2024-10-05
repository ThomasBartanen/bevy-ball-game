use bevy::prelude::*;

pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .init_resource::<Kills>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                Update,
                (
                    (update_score, update_kills).run_if(in_state(AppState::Game)),
                    update_high_scores,
                    high_scores_updated,
                ),
            )
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
