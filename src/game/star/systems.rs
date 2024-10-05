use super::components::*;
use super::resources::*;
use crate::constants::*;
use crate::extension_functions::get_random_screen_point;

use bevy::{prelude::*, window::PrimaryWindow};
//use rand::prelude::*;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(get_random_screen_point(window)),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_star(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window: &Window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(get_random_screen_point(window)),
                texture: asset_server.load(STAR_SPRITE_PATH),
                ..default()
            },
            Star {},
        ));
    }
}
