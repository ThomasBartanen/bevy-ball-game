use super::resources::*;
use crate::{constants::ENEMY_KILL_SCORE, events::*};

use bevy::prelude::*;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
    commands.insert_resource(Kills::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
    commands.remove_resource::<Kills>();
}

pub fn update_score(mut score: ResMut<Score>, mut enemy_killed_event: EventReader<EnemyKilled>) {
    for _event in enemy_killed_event.read() {
        score.value += ENEMY_KILL_SCORE;
    }
}

pub fn update_kills(mut kills: ResMut<Kills>, mut enemy_killed_event: EventReader<EnemyKilled>) {
    for _event in enemy_killed_event.read() {
        kills.value += 1;
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores
            .scores
            .push(("Player".to_string(), event.score, event.kills));
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores: {:?}", high_scores);
    }
}
