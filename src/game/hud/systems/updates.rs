use bevy::prelude::*;

use crate::game::{
    hud::components::{ScoreText, CurrencyText, BombCount, KillText},
    score::resources::{Score, Kills},
    currency::resources::HeldCurrency, bomb::resources::HeldBombCount
};

pub fn update_score(
    mut score_query: Query<&mut Text, With<ScoreText>>,
    score: Res<Score>
) {
    for mut text in &mut score_query {
        text.sections[0].value = "Score: ".to_owned() + score.value.to_string().as_str();
    }
}

pub fn update_currency(
    mut currency_query: Query<&mut Text, With<CurrencyText>>,
    held_currency: Res<HeldCurrency>
) {
    for mut text in &mut currency_query {
        text.sections[0].value = "Currency: ".to_owned() + held_currency.amount.to_string().as_str();
    }
}

pub fn update_kill_count(
    mut kill_query: Query<&mut Text, With<KillText>>,
    kills: Res<Kills>
) {
    for mut text in &mut kill_query {
        text.sections[0].value = "Kills: ".to_owned() + kills.value.to_string().as_str();
    }
}

pub fn update_bombs(
    mut bomb_query: Query<&mut Text, With<BombCount>>,
    held_bombs: Res<HeldBombCount>
) {
    for mut text in &mut bomb_query {
        text.sections[0].value = "Bombs: ".to_owned() + held_bombs.count.to_string().as_str();
    }
}