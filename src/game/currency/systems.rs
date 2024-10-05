use bevy::prelude::*;

use crate::constants::{BOMB_COST, BUY_BOMB_KEY, CURRENCY_PER_KILL};
use crate::events::EnemyKilled;
use crate::game::bomb::resources::HeldBombCount;

use super::resources::HeldCurrency;

pub fn insert_currency(mut commands: Commands) {
    commands.insert_resource(HeldCurrency::default());
}

pub fn remove_currency(mut commands: Commands) {
    commands.remove_resource::<HeldCurrency>();
}

pub fn purchase_bomb(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut held_currency: ResMut<HeldCurrency>,
    mut held_bomb_counter: ResMut<HeldBombCount>,
) {
    if keyboard_input.just_pressed(BUY_BOMB_KEY) {
        if held_currency.amount >= BOMB_COST {
            println!(
                "Purchased Bomb. {} currency remaining",
                held_currency.amount
            );
            held_bomb_counter.count += 1;
            held_currency.amount -= BOMB_COST;
        } else {
            println!(
                "Not enough cash to buy bomb. Only {} held, and {} required",
                held_currency.amount, BOMB_COST
            );
        }
    }
}

pub fn get_currency_on_kill(
    mut currency: ResMut<HeldCurrency>,
    mut enemy_killed_event: EventReader<EnemyKilled>,
) {
    for _event in enemy_killed_event.read() {
        currency.amount += CURRENCY_PER_KILL;
    }
}
