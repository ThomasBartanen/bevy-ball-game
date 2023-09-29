use bevy::prelude::*;

#[derive(Resource)]
pub struct  Score{
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score{
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32, u32)>
}

impl Default for HighScores {
    fn default() -> HighScores {
        HighScores {
            scores: Vec::new()
        }
    }
}

#[derive(Resource)]
pub struct  Kills{
    pub value: u32,
}

impl Default for Kills {
    fn default() -> Kills{
        Kills { value: 0 }
    }
}