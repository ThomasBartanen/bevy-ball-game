use bevy::prelude::*;
use crate::constants::*;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer { 
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) 
        }
    }
}

#[derive(Resource)]
pub struct EnemyCount {
    pub count: usize
}

impl Default for EnemyCount {
    fn default() -> EnemyCount {
        EnemyCount { count: 0 }
    }
}