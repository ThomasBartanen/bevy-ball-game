use crate::AppState;
use bevy::prelude::*;

use super::sound::{resources::*, systems::*};

mod components;
pub mod resources;
pub mod systems;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SFXValues>()
            .init_resource::<MusicValues>()
            .init_resource::<SFXQueue>()
            .add_systems(OnEnter(AppState::MainMenu), initialize_music)
            .add_systems(
                Update,
                (
                    play_queued_sfx,
                    queue_collect_sound,
                    queue_random_bounce_sound,
                )
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(OnEnter(AppState::GameOver), despawn_music_entity);
    }
}
