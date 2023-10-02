use bevy::{prelude::*, audio::{PlaybackMode, Volume, VolumeLevel}};

#[derive(Resource)]
pub struct SFXQueue{
    pub bounces_requested: u16,
    pub collects_requested: u16,
    pub sound_queue: Vec<AudioBundle>
}

impl Default for SFXQueue {
    fn default() -> SFXQueue{
        SFXQueue { 
            bounces_requested: 0,
            collects_requested: 0,
            sound_queue: Vec::new()
        }
    }
}

#[derive(Resource)]
pub struct  SFXValues{
    pub settings: PlaybackSettings,
}

impl Default for SFXValues {
    fn default() -> SFXValues{
        SFXValues { 
            settings: PlaybackSettings { 
                mode: PlaybackMode::Once, 
                volume: Volume::Relative(VolumeLevel::new(0.1)), 
                ..default()
            }
        }
    }
}

#[derive(Resource)]
pub struct  MusicValues{
    pub settings: PlaybackSettings,
}

impl Default for MusicValues {
    fn default() -> MusicValues{
        MusicValues { 
            settings: PlaybackSettings { 
                mode: PlaybackMode::Loop, 
                volume: Volume::Relative(VolumeLevel::new(0.05)), 
                ..default()
            }
        }
    }
}