use bevy::prelude::*;

use crate::constants::BOMB_COST;
use crate::game::bomb::resources::HeldBombCount;

use super::resources::HeldCurrency;

pub fn replenish_bomb_count(
    keyboard_input: Res<Input<KeyCode>>,
    mut held_currency: ResMut<HeldCurrency>,
    mut held_bomb_counter: ResMut<HeldBombCount>
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        if held_currency.amount >= BOMB_COST {
            println!("Purchased Bomb. {} currency remaining", held_currency.amount);
            held_bomb_counter.count += 1;
            held_currency.amount -= BOMB_COST;
        }
        else {
            println!("Not enough cash to buy bomb. Only {} held, and {} required", held_currency.amount, BOMB_COST);
        }
    }
}