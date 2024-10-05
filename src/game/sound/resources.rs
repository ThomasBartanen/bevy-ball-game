use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

#[derive(Resource, Default)]
pub struct SFXQueue {
    pub bounces_requested: u16,
    pub collects_requested: u16,
    pub sound_queue: Vec<AudioBundle>,
}

#[derive(Resource)]
pub struct SFXValues {
    pub settings: PlaybackSettings,
}

impl Default for SFXValues {
    fn default() -> SFXValues {
        SFXValues {
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(0.1),
                ..default()
            },
        }
    }
}

#[derive(Resource)]
pub struct MusicValues {
    pub settings: PlaybackSettings,
}

impl Default for MusicValues {
    fn default() -> MusicValues {
        MusicValues {
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new(0.15),
                ..default()
            },
        }
    }
}
