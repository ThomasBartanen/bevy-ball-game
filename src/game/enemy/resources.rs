use bevy::prelude::*;
use crate::constants::*;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { 
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}